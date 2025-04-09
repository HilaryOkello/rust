pub struct One {
    first_layer: Option<Two>
}


pub struct Two {
    second_layer: Option<Three>
}


pub struct Three {
    third_layer: Option<Four>
}


pub struct Four {
    fourth_layer: Option<u16>
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        let two = self.first_layer?;
        let three = two.second_layer?;
        let four = three.third_layer?;
        four.fourth_layer
    }
}