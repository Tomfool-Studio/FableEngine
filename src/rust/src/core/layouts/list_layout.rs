use uuid::Uuid;

use crate::core::layouts::layout::Layout;

pub struct ListLayout {
    name: String,
    ordered_list: Vec<Uuid>
}

impl ListLayout {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), ordered_list: Vec::new() }
    }
}

impl super::layout::Layout for ListLayout {
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn layout(&self) -> &[Uuid] {
        &self.ordered_list
    }
    fn len(&self) -> usize {
        self.ordered_list.len()
    }

    fn copy_to(&self, target: &mut dyn Layout, start_index: usize) {
        for (index, uuid) in self.ordered_list.iter().enumerate() {
            let _ = target.add_id(*uuid, start_index+index);
        }
    }

    fn add_id(&mut self, uuid: Uuid, index: usize) -> Result<(), String> {
        if index > self.ordered_list.len() {
            return Err(format!("Index {} is out of bounds. Valid range is [0, {}].", index, self.ordered_list.len()));
        }

        self.ordered_list.insert(index, uuid);
        Ok(())
    }

    fn remove_id(&mut self, uuid: Uuid) -> Result<usize, String> {
        let index = self.ordered_list.iter().position(|cur_uuid| *cur_uuid == uuid).ok_or(format!("UUID {} could not be found in module list", uuid))?;

        self.ordered_list.remove(index);
        
        Ok(index)
    }
    fn remove_id_by_index(&mut self, index: usize) -> Result<Uuid, String> {
        if index <= self.ordered_list.len() {
            Ok(self.ordered_list.remove(index))
        } else {
            Err(format!("Index {} is out of bounds. Valid range is [0, {}].", index, self.ordered_list.len()))
        }
    }

    fn move_id(&mut self, uuid: Uuid, new_index: usize) -> Result<(), String> {
        if new_index > self.ordered_list.len() {
            return Err(format!("Index {} is out of bounds. Valid range is [0, {}].", new_index, self.ordered_list.len()));
        }
        
        let old_index = self.ordered_list.iter().position(|cur_uuid| *cur_uuid == uuid).ok_or(format!("UUID {} could not be found in module list", uuid))?;

        if old_index < new_index {
            self.ordered_list[old_index..=new_index].rotate_left(1);
        } else if old_index > new_index {
            self.ordered_list[new_index..=old_index].rotate_right(1);
        }

        Ok(())
    }
    fn move_id_by_index(&mut self, old_index: usize, new_index: usize) -> Result<(), String> {
        if old_index > self.ordered_list.len() {
            return Err(format!("Old index {} is out of bounds. Valid range is [0, {}].", old_index, self.ordered_list.len()));
        }
        if new_index > self.ordered_list.len() {
            return Err(format!("New index {} is out of bounds. Valid range is [0, {}].", new_index, self.ordered_list.len()));
        }
        
        if old_index < new_index {
            self.ordered_list[old_index..=new_index].rotate_left(1);
        } else if old_index > new_index {
            self.ordered_list[new_index..=old_index].rotate_right(1);
        }

        Ok(())
    }
}

crate::impl_reflection!(ListLayout,);
impl Default for ListLayout {
    fn default() -> Self {
        Self {
            name: String::from("List of someting"),
            ordered_list: Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::layout::Layout;
    use uuid::Uuid;

    fn sample_ids(n: usize) -> Vec<Uuid> {
        (0..n).map(|_| Uuid::new_v4()).collect()
    }

    #[test]
    fn add_ok_err() {
        let mut l = ListLayout { name: "t".into(), ordered_list: Vec::new() };
        let ids = sample_ids(2);
        assert!(l.add_id(ids[0], 0).is_ok());
        assert!(l.add_id(ids[1], 5).is_err()); // out of bounds
        assert_eq!(l.ordered_list, vec![ids[0]]);
    }

    #[test]
    fn remove_by_id_ok_err() {
        let ids = sample_ids(2);
        let mut l = ListLayout { name: "t".into(), ordered_list: ids.clone() };
        assert_eq!(l.remove_id(ids[0]).unwrap(), 0);
        assert!(l.remove_id(Uuid::new_v4()).is_err()); // not present
        assert_eq!(l.ordered_list, vec![ids[1]]);
    }

    #[test]
    fn remove_by_index_ok_err() {
        let ids = sample_ids(2);
        let mut l = ListLayout { name: "t".into(), ordered_list: ids.clone() };
        assert_eq!(l.remove_id_by_index(1).unwrap(), ids[1]);
        assert!(l.remove_id_by_index(5).is_err());
        assert_eq!(l.ordered_list, vec![ids[0]]);
    }

    #[test]
    fn move_by_id_ok_err() {
        let ids = sample_ids(3);
        let mut l = ListLayout { name: "t".into(), ordered_list: ids.clone() };
        assert!(l.move_id(ids[0], 2).is_ok()); // move to end
        assert_eq!(l.ordered_list, vec![ids[1], ids[2], ids[0]]);
        assert!(l.move_id(Uuid::new_v4(), 1).is_err()); // missing id
        assert!(l.move_id(ids[0], 99).is_err()); // out of bounds
    }

    #[test]
    fn move_by_index_ok_err() {
        let ids = sample_ids(3);
        let mut l = ListLayout { name: "t".into(), ordered_list: ids.clone() };
        assert!(l.move_id_by_index(0, 2).is_ok());
        assert_eq!(l.ordered_list, vec![ids[1], ids[2], ids[0]]);
        assert!(l.move_id_by_index(9, 1).is_err());
        assert!(l.move_id_by_index(0, 9).is_err());
    }

    #[test]
    fn copy_to_other() {
        let ids = sample_ids(3);
        let src = ListLayout { name: "src".into(), ordered_list: ids.clone() };
        let mut dst = ListLayout { name: "dst".into(), ordered_list: Vec::new() };
        src.copy_to(&mut dst, 0);
        assert_eq!(dst.ordered_list, ids);
        assert_eq!(src.ordered_list, ids); // copy, not drain
    }
}
