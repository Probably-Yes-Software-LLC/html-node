#![allow(non_snake_case)]

use crate::typed_element;

typed_element! {
    pub path("path") {
        d,
        pathLength
    }
}

typed_element! {
    pub rect("rect") {
        x,
        y,
        width,
        height,
        rx,
        ry,
        pathLength
    }
}
