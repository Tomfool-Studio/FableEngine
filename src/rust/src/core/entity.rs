use super::modules::module::Module;
use super::layouts::{layout::Layout, list_layout::ListLayout};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Entity {
    name: String,
    module_map: HashMap<Uuid, Box<dyn Module>>,
    parent_map: HashMap<Uuid, Uuid>,
    layout_map: HashMap<Uuid, Box<dyn Layout>>,
    root_layout_id: Uuid,
}

impl Entity {
    pub fn new(name: &str) -> Self {
        let root_uuid: Uuid = Uuid::new_v4();
        let mut layout_hash_map: HashMap<Uuid, Box<dyn Layout>> = HashMap::new();
        layout_hash_map.insert(root_uuid, Box::new(ListLayout::new(name)));

        Self {
            name: name.to_string(),
            module_map: HashMap::new(),
            parent_map: HashMap::new(),
            layout_map: layout_hash_map,
            root_layout_id: root_uuid,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn root_layout_id(&self) -> &Uuid {
        &self.root_layout_id
    }
    
    // module 
    pub fn module(&self, module_id: Uuid) -> Option<&dyn Module> {
        if let Some(module) = self.module_map.get(&module_id) {
            Some(module.as_ref())
        } else {
            None
        }
    }

    pub fn module_mut(&mut self, module_id: Uuid) -> Option<&mut dyn Module> {
        if let Some(module) = self.module_map.get_mut(&module_id) {
            Some(module.as_mut())
        } else {
            None
        }
    }

    pub fn add_module<T: Module + 'static>(&mut self, module: T, layout_id: Uuid, index: usize) -> Result<Uuid, String> { 
        let layout = self.layout_map.get_mut(&layout_id).ok_or(format!("Layout with UUID {} not found", layout_id))?;
        let new_uuid = Uuid::new_v4();

        layout.add_id(new_uuid, index)?;
        self.module_map.insert(new_uuid, Box::new(module));
        self.parent_map.insert(new_uuid, layout_id);
        
        Ok(new_uuid)
    }

    pub fn add_module_to_root<T: Module + 'static>(&mut self, module: T, index: usize) -> Result<Uuid, String> {
        self.add_module(module, self.root_layout_id, index)
    }

    pub fn remove_module(&mut self, module_id: Uuid) -> Result<Box<dyn Module>, String> {
        self.remove_module_internal(module_id, true)
    }

    fn remove_module_internal(&mut self, module_id: Uuid, parent_remove: bool) -> Result<Box<dyn Module>, String> {
        let module = self.module_map.remove(&module_id).ok_or(format!("Module with UUID {} not found", module_id))?;
        
        let parent_id: Uuid = self.parent_map.remove(&module_id).unwrap();

        if parent_remove {
            let parent_layout: &mut Box<dyn Layout> = self.layout_map.get_mut(&parent_id).unwrap();
            let _ = parent_layout.remove_id(module_id);
        }

        Ok(module)
    }


    // layout
    pub fn layout(&self, layout_id: Uuid) -> Option<&dyn Layout>{
        if let Some(boxed_layout) = self.layout_map.get(&layout_id) {
            Some(boxed_layout.as_ref())
        } else {
            None
        }
    }

    pub fn set_layout_name(&mut self, layout_id: Uuid, name: &str) -> Result<(), String>{
        let layout = self.layout_map.get_mut(&layout_id).ok_or(format!("Layout with UUID {} not found", layout_id))?;
        layout.set_name(name);

        Ok(())
    }

    pub fn add_layout(&mut self, layout_name: &str, layout_type: LayoutType, parent_layout_id: Uuid, index: usize) -> Result<Uuid, String> { 
        let parent_layout = self.layout_map.get_mut(&parent_layout_id).ok_or(format!("Layout with UUID {} not found", parent_layout_id))?;
        let new_uuid = Uuid::new_v4();
        
        parent_layout.add_id(new_uuid, index)?;
        self.layout_map.insert(new_uuid, LayoutType::create_layout(layout_type, layout_name));
        self.parent_map.insert(new_uuid, parent_layout_id);
        
        Ok(new_uuid)
    }

    pub fn add_layout_to_root(&mut self, layout_name: &str, layout_type: LayoutType, index: usize) -> Result<Uuid, String> { 
        self.add_layout(layout_name, layout_type, self.root_layout_id, index)
    }

    pub fn remove_layout(&mut self, layout_id: Uuid) -> Result<Box<dyn Layout>, String> {
        if layout_id == self.root_layout_id {
            return Err("Cannot remove root layout".to_string());
        }

        let layout = self.layout_map.remove(&layout_id).ok_or(format!("Layout with UUID {} not found", layout_id))?;

        let parent_id = self.parent_map.remove(&layout_id).unwrap();
        let parent_layout = self.layout_map.get_mut(&parent_id).unwrap();

        let layout_index = parent_layout.remove_id(layout_id).unwrap();
        
        layout.copy_to(parent_layout.as_mut(), layout_index);
        
        Ok(layout)
    }

    pub fn remove_layout_and_children(&mut self, layout_id: Uuid) -> Result<LayoutTree, String> {
        if layout_id == self.root_layout_id {
            return Err("Cannot remove root layout".to_string());
        }

        let removed_layout = self.layout_map.remove(&layout_id).ok_or(format!("Layout with UUID {} not found", layout_id))?;

        let mut layout_tree: Vec<LayoutTree> = Vec::new();
        for child_id in removed_layout.layout() {
            if self.module_map.contains_key(child_id) {
                layout_tree.push(LayoutTree::Module(self.remove_module_internal(*child_id, false)?));
            } else {
                layout_tree.push(self.remove_layout_and_children(*child_id)?);
            }
        }

        let parent_id = self.parent_map.remove(&layout_id).unwrap();
        let parent_layout = self.layout_map.get_mut(&parent_id).unwrap();

        parent_layout.remove_id(layout_id).unwrap();

        Ok(LayoutTree::Layout{name: removed_layout.name().to_string(), list: layout_tree })
    }

    pub fn change_layout_to(&mut self, layout_type: LayoutType, layout_id: Uuid) -> Result<(), String>{
        let old_layout = self.layout_map.get_mut(&layout_id).ok_or(format!("Layout with UUID {} not found", layout_id))?;
        let mut new_layout = LayoutType::create_layout(layout_type, old_layout.name());
        
        old_layout.copy_to(new_layout.as_mut(), 0);
        self.layout_map.insert(layout_id, new_layout);
        
        Ok(())
    }

    // shared
    pub fn move_within_layout(&mut self, child_id: Uuid, index: usize) -> Result<(), String> {
        let cur_layout_id = self.parent_map.get(&child_id).ok_or(format!("Module/Layout with UUID {} not found", child_id))?;
        let cur_layout = self.layout_map.get_mut(&cur_layout_id).unwrap();

        cur_layout.move_id(child_id, index)?;

        Ok(())
    }

    pub fn move_to_layout(&mut self, child_id: Uuid, layout_id: Uuid, index: usize) -> Result<(), String> {
        let cur_layout_id = self.parent_map.get(&child_id).ok_or(format!("Module/Layout with UUID {} not found", child_id))?;

        let new_layout = self.layout_map.get_mut(&layout_id).ok_or(format!("Layout with UUID {} not found", child_id))?;
        new_layout.add_id(child_id, index)?;

        let cur_layout = self.layout_map.get_mut(&cur_layout_id).unwrap();
        let _ = cur_layout.remove_id(child_id);

        self.parent_map.insert(child_id, layout_id);

        Ok(())
    }
}

pub enum LayoutType {
    List,
}
impl LayoutType {
    fn create_layout(layout_type: LayoutType, layout_name: &str) -> Box<dyn Layout> {
        match layout_type {
            LayoutType::List => Box::new(ListLayout::new(layout_name)),
        }
    }
}

pub enum LayoutTree {
    Module(Box<dyn Module>),
    Layout{
        name: String,
        list: Vec<LayoutTree>,
    }
}

// Entity Modules
pub mod modules {
    pub use super::super::modules::module::Module;
    pub use super::super::modules::death_saves_module::DeathSaveModule;
    pub use super::super::modules::dice_pool_module::DicePoolModule;
    pub use super::super::modules::health_module::HealthModule;
    pub use super::super::modules::stat_module::StatModule;
    pub use super::super::modules::text_module::TextModule;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_reflection;

    #[derive(Debug)]
    struct DummyModule { value: i32 }
    impl Module for DummyModule {}
    impl_reflection!(DummyModule,);

    fn new_entity() -> Entity {
        Entity::new("root")
    }

    #[test]
    fn new_entity_has_root() {
        let e = new_entity();
        assert_eq!(e.name(), "root");
        assert!(e.layout_map.contains_key(e.root_layout_id()));
    }

    #[test]
    fn module_add_and_remove() {
        let mut e = new_entity();
        let m_id = e.add_module_to_root(DummyModule { value: 1 }, 0).unwrap();

        assert!(e.module(m_id).is_some());
        assert_eq!(e.parent_map.get(&m_id), Some(e.root_layout_id()));

        let removed = e.remove_module(m_id).unwrap();
        assert_eq!(removed.downcast_module::<DummyModule>().unwrap().value, 1);
        assert!(e.module(m_id).is_none());
    }

    #[test]
    fn layout_add_and_remove() {
        let mut e = new_entity();
        let l_id = e.add_layout_to_root("child", LayoutType::List, 0).unwrap();

        assert!(e.layout(l_id).is_some());
        assert_eq!(e.parent_map.get(&l_id), Some(e.root_layout_id()));

        let removed = e.remove_layout(l_id).unwrap();
        assert_eq!(removed.name(), "child");
        assert!(e.layout(l_id).is_none());
    }

    #[test]
    fn cannot_remove_root_layout() {
        let mut e = new_entity();
        let result = e.remove_layout(*e.root_layout_id());
        assert!(result.is_err());
    }

    #[test]
    fn change_layout_type_preserves_children() {
        let mut e = new_entity();
        let m1 = e.add_module_to_root(DummyModule { value: 1 }, 0).unwrap();
        let m2 = e.add_module_to_root(DummyModule { value: 2 }, 1).unwrap();

        e.change_layout_to(LayoutType::List, *e.root_layout_id()).unwrap();

        let layout = e.layout(*e.root_layout_id()).unwrap();
        assert_eq!(layout.len(), 2);
        assert!(layout.layout().contains(&m1));
        assert!(layout.layout().contains(&m2));
    }

    #[test]
    fn move_within_and_between_layouts() {
        let mut e = new_entity();
        let m1 = e.add_module_to_root(DummyModule { value: 1 }, 0).unwrap();
        let l2 = e.add_layout_to_root("child", LayoutType::List, 1).unwrap();

        // move within
        e.move_within_layout(m1, 0).unwrap();

        // move to another layout
        e.move_to_layout(m1, l2, 0).unwrap();
        assert_eq!(e.parent_map.get(&m1), Some(&l2));
        let child_layout = e.layout(l2).unwrap();
        assert_eq!(child_layout.layout()[0], m1);
    }

    #[test]
    fn remove_layout_and_children_removes_all() {
        let mut e = new_entity();
        let l_id = e.add_layout_to_root("child", LayoutType::List, 0).unwrap();
        let m_id = e.add_module(DummyModule { value: 7 }, l_id, 0).unwrap();

        let tree = e.remove_layout_and_children(l_id).unwrap();

        // layout & child module removed from maps
        assert!(e.layout(l_id).is_none());
        assert!(e.module(m_id).is_none());

        // returned tree preserves data
        match tree {
            LayoutTree::Layout { name, list } => {
                assert_eq!(name, "child");
                assert!(matches!(list[0], LayoutTree::Module(_)));
            }
            _ => panic!("wrong tree"),
        }
    }
}
