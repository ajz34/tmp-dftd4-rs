use crate::prelude::*;
use std::ffi::{c_char, c_double, c_int};

pub unsafe fn calc_dftd4_rest_(
    num: *const c_int,
    num_size: *const c_int,
    xyz: *const c_double,
    charge: *const c_double,
    uhf: *const c_int,
    method: *const c_char,
    method_len: *const c_int,
    energy: *mut c_double,
    gradient: *mut c_double,
    sigma: *mut c_double,
) {
    let _ = uhf;

    // convert c-style arguments to rust-style arguments
    let natoms = unsafe { *num_size } as usize;
    let charges = {
        let charges = unsafe { std::slice::from_raw_parts(num, natoms) };
        charges.iter().map(|&x| x as usize).collect::<Vec<usize>>()
    };
    let coords = {
        let coords = unsafe { std::slice::from_raw_parts(xyz, natoms * 3) };
        coords.iter().map(|&x| x).collect::<Vec<f64>>() // this may not required, since c_double is always f64
    };
    let method = {
        let method = unsafe { std::slice::from_raw_parts(method, *method_len as usize) };
        let method = method.iter().map(|&x| x as u8).collect::<Vec<u8>>();
        std::str::from_utf8(&method).unwrap().to_string()
    };
    let charge = match charge.is_null() {
        true => None,
        false => Some(*charge),
    };

    // create structure and model
    let structure = DFTD4Structure::new(natoms, &charges, &coords, charge, None, None);
    let model = DFTD4Model::new(&structure);

    // get dispersion energy and gradient
    let param = DFTD4Param::load_rational_damping(&method, true);
    let result = get_dispersion(&structure, &model, &param, true, true);

    // set energy and gradient
    unsafe {
        *energy = result.0;
        let gradient = std::slice::from_raw_parts_mut(gradient, natoms * 3);
        let sigma = std::slice::from_raw_parts_mut(sigma, 3 * 3);
        gradient.copy_from_slice(&result.1.unwrap());
        sigma.copy_from_slice(&result.2.unwrap());
    }
}
