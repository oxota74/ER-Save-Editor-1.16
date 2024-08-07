use std::{cell::RefCell, rc::Rc};

use er_save_lib::EquipParamWeapon::EquipParamWeapon;

use super::item_param::ItemParam;

pub(crate) struct ParamFilter;

impl ParamFilter {
    pub(crate) fn not_infused(weapon: &&Rc<RefCell<ItemParam<EquipParamWeapon>>>) -> bool {
        weapon
            .as_ref()
            .borrow()
            .header
            .as_ref()
            .borrow()
            .item_id
            .as_ref()
            .borrow()
            .to_owned()
            % 10_000
            == 0
    }
}
