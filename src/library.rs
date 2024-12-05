use crate::ffi;
use std::ffi::{c_char, c_int, CStr};
use std::ptr::{null, null_mut};
use std::result::Result;

/// Get the version of the DFTD4 library.
pub fn get_api_version() -> String {
    let version = unsafe { ffi::dftd4_get_version() };
    format!(
        "{}.{}.{}",
        version / 10000,
        version / 100 % 100,
        version % 100
    )
}

/// Get the version of the DFTD4 library in list of integers.
pub fn get_api_version_compact() -> [usize; 3] {
    let version = unsafe { ffi::dftd4_get_version() } as usize;
    [version / 10000, version / 100 % 100, version % 100]
}

pub enum DFTD4Error {
    C(ffi::dftd4_error),
    Rust(String),
}

impl Drop for DFTD4Error {
    fn drop(&mut self) {
        match self {
            DFTD4Error::C(ptr) => unsafe { ffi::dftd4_delete_error(&mut ptr.clone()) },
            DFTD4Error::Rust(_) => (),
        }
    }
}

impl DFTD4Error {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::dftd4_new_error() };
        DFTD4Error::C(ptr)
    }

    pub fn check(&self) -> bool {
        match self {
            DFTD4Error::C(ptr) => unsafe { ffi::dftd4_check_error(*ptr) != 0 },
            DFTD4Error::Rust(_) => true,
        }
    }

    pub fn get_c_ptr(&mut self) -> ffi::dftd4_error {
        match self {
            DFTD4Error::C(ptr) => *ptr,
            DFTD4Error::Rust(_) => std::ptr::null_mut(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            DFTD4Error::C(ptr) => {
                const LEN_BUFFER: usize = 512;
                let buffer = [0u8; LEN_BUFFER];
                let raw = buffer.as_ptr() as *mut c_char;
                let msg = unsafe {
                    ffi::dftd4_get_error(*ptr, raw, &(LEN_BUFFER as c_int));
                    CStr::from_ptr(raw)
                };
                return msg.to_string_lossy().to_string();
            }
            DFTD4Error::Rust(msg) => msg.clone(),
        }
    }
}

impl std::fmt::Debug for DFTD4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD4Error: {}", self.get_message())
        } else {
            write!(f, "DFTD4Error: No error")
        }
    }
}

impl std::fmt::Display for DFTD4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD4Error: {}", self.get_message())
        } else {
            write!(f, "")
        }
    }
}

impl std::error::Error for DFTD4Error {}

pub struct DFTD4Structure {
    ptr: ffi::dftd4_structure,
    natoms: usize,
}

impl Drop for DFTD4Structure {
    fn drop(&mut self) {
        unsafe { ffi::dftd4_delete_structure(&mut self.ptr) };
    }
}

impl DFTD4Structure {
    /// Get number of atoms
    pub fn get_natoms(&self) -> usize {
        self.natoms
    }

    /// Create new molecular structure data (quantities in Bohr) (failable)
    pub fn new_f(
        natoms: usize,
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD4Error> {
        // check dimension
        if numbers.len() != natoms {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for numbers, expected {}, got {}",
                natoms,
                numbers.len()
            )));
        }
        if positions.len() != 3 * natoms {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * natoms,
                positions.len()
            )));
        }
        if lattice.is_some_and(|lattice| lattice.len() != 9) {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.unwrap().len()
            )));
        }
        // unwrap optional values
        let charge_ptr = charge.map_or(null(), |x| &x as *const f64);
        let lattice_ptr = lattice.map_or(null(), |x| x.as_ptr());
        let periodic_ptr = periodic.map_or(null(), |x| x.as_ptr());
        // type conversion from usual definitions
        let natoms_c_int = natoms as c_int;
        let atomic_numbers = numbers.iter().map(|&x| x as c_int).collect::<Vec<c_int>>();
        // actual driver for creating the structure
        let mut error = DFTD4Error::new();
        let ptr = unsafe {
            ffi::dftd4_new_structure(
                error.get_c_ptr(),
                natoms_c_int,
                atomic_numbers.as_ptr(),
                positions.as_ptr(),
                charge_ptr,
                lattice_ptr,
                periodic_ptr,
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, natoms }),
        }
    }

    /// Create new molecular structure data (quantities in Bohr)
    ///
    /// # Arguments
    ///
    /// * `numbers` - numbers [natoms]
    /// * `positions` - positions [natoms][3]
    /// * `lattice` - lattice [3][3]
    /// * `periodic` - periodic [3]
    pub fn new(
        natoms: usize,
        numbers: &[usize],
        positions: &[f64],
        charge: Option<f64>,
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::new_f(natoms, numbers, positions, charge, lattice, periodic).unwrap()
    }

    /// Update coordinates and lattice parameters (quantities in Bohr) (failable)
    pub fn update_f(&self, positions: &[f64], lattice: Option<&[f64]>) -> Result<(), DFTD4Error> {
        // check dimension
        if positions.len() != 3 * self.natoms {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * self.natoms,
                positions.len()
            )));
        }
        if lattice.is_some_and(|lattice| lattice.len() != 9) {
            return Err(DFTD4Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.unwrap().len()
            )));
        }
        // unwrap optional values
        let lattice_ptr = lattice.map_or(null(), |x| x.as_ptr());
        // actual driver for updating the structure
        let mut error = DFTD4Error::new();
        unsafe {
            ffi::dftd4_update_structure(
                error.get_c_ptr(),
                self.ptr,
                positions.as_ptr(),
                lattice_ptr,
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }

    /// Update coordinates and lattice parameters (quantities in Bohr)
    ///
    /// # Arguments
    ///
    /// * `positions` - positions [natoms][3]
    /// * `lattice` - lattice [3][3]
    pub fn update(&self, positions: &[f64], lattice: Option<&[f64]>) {
        self.update_f(positions, lattice).unwrap()
    }
}

pub struct DFTD4Model {
    ptr: ffi::dftd4_model,
}

impl Drop for DFTD4Model {
    fn drop(&mut self) {
        unsafe { ffi::dftd4_delete_model(&mut self.ptr) };
    }
}

impl DFTD4Model {
    /// Create new D4 dispersion model (failable)
    pub fn new_f(structure: &DFTD4Structure) -> Result<Self, DFTD4Error> {
        let mut error = DFTD4Error::new();
        let ptr = unsafe { ffi::dftd4_new_d4_model(error.get_c_ptr(), structure.ptr) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new D4 dispersion model
    pub fn new(structure: &DFTD4Structure) -> Self {
        Self::new_f(structure).unwrap()
    }

    /// Create new D4 dispersion model (failable)
    pub fn custom_f(
        structure: &DFTD4Structure,
        ga: f64,
        gc: f64,
        gf: f64,
    ) -> Result<Self, DFTD4Error> {
        let mut error = DFTD4Error::new();
        let ptr =
            unsafe { ffi::dftd4_custom_d4_model(error.get_c_ptr(), structure.ptr, ga, gc, gf) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new D4 dispersion model
    pub fn custom(structure: &DFTD4Structure, ga: f64, gc: f64, gf: f64) -> Self {
        Self::custom_f(structure, ga, gc, gf).unwrap()
    }
}

pub struct DFTD4Param {
    ptr: ffi::dftd4_param,
}

impl Drop for DFTD4Param {
    fn drop(&mut self) {
        unsafe { ffi::dftd4_delete_param(&mut self.ptr) };
    }
}

impl DFTD4Param {
    /// Create new rational damping parameters (failble)
    pub fn new_rational_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
    ) -> Result<Self, DFTD4Error> {
        let mut error = DFTD4Error::new();
        let ptr =
            unsafe { ffi::dftd4_new_rational_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new rational damping parameters
    pub fn new_rational_damping(s6: f64, s8: f64, s9: f64, a1: f64, a2: f64, alp: f64) -> Self {
        Self::new_rational_damping_f(s6, s8, s9, a1, a2, alp).unwrap()
    }

    /// Load rational damping parameters from internal storage (failble)
    pub fn load_rational_damping_f(method: &str, mdb: bool) -> Result<Self, DFTD4Error> {
        let mut error = DFTD4Error::new();
        let name_c = std::ffi::CString::new(method).unwrap();
        let ptr = unsafe {
            ffi::dftd4_load_rational_damping(error.get_c_ptr(), name_c.as_ptr() as *mut c_char, mdb)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load rational damping parameters from internal storage
    pub fn load_rational_damping(method: &str, mdb: bool) -> Self {
        Self::load_rational_damping_f(method, mdb).unwrap()
    }
}

/// Evaluate properties related to the dispersion model (failable)
pub fn get_properties_f(
    structure: &DFTD4Structure,
    model: &DFTD4Model,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), DFTD4Error> {
    let mut error = DFTD4Error::new();
    let natoms = structure.get_natoms();
    let mut cn = vec![0.0; natoms];
    let mut charges = vec![0.0; natoms];
    let mut c6 = vec![0.0; natoms * natoms];
    let mut alpha = vec![0.0; natoms];
    unsafe {
        ffi::dftd4_get_properties(
            error.get_c_ptr(),
            structure.ptr,
            model.ptr,
            cn.as_mut_ptr(),
            charges.as_mut_ptr(),
            c6.as_mut_ptr(),
            alpha.as_mut_ptr(),
        )
    };
    match error.check() {
        true => Err(error),
        false => Ok((cn, charges, c6, alpha)),
    }
}

/// Evaluate properties related to the dispersion model
pub fn get_properties(
    structure: &DFTD4Structure,
    model: &DFTD4Model,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    get_properties_f(structure, model).unwrap()
}

/// Evaluate the dispersion energy and its derivative (failable)
pub fn get_dispersion_f(
    structure: &DFTD4Structure,
    model: &DFTD4Model,
    param: &DFTD4Param,
    eval_grad: bool,
    eval_sigma: bool,
) -> Result<(f64, Option<Vec<f64>>, Option<Vec<f64>>), DFTD4Error> {
    let natoms = structure.get_natoms();
    let mut energy = 0.0;
    let mut grad = match eval_grad {
        true => Some(vec![0.0; 3 * natoms]),
        false => None,
    };
    let mut sigma = match eval_sigma {
        true => Some(vec![0.0; 9]),
        false => None,
    };
    let mut error = DFTD4Error::new();
    unsafe {
        ffi::dftd4_get_dispersion(
            error.get_c_ptr(),
            structure.ptr,
            model.ptr,
            param.ptr,
            &mut energy,
            grad.as_mut().map_or(null_mut(), |x| x.as_mut_ptr()),
            sigma.as_mut().map_or(null_mut(), |x| x.as_mut_ptr()),
        )
    };
    match error.check() {
        true => Err(error),
        false => Ok((energy, grad, sigma)),
    }
}

/// Evaluate the dispersion energy and its derivative
pub fn get_dispersion(
    structure: &DFTD4Structure,
    model: &DFTD4Model,
    param: &DFTD4Param,
    eval_grad: bool,
    eval_sigma: bool,
) -> (f64, Option<Vec<f64>>, Option<Vec<f64>>) {
    get_dispersion_f(structure, model, param, eval_grad, eval_sigma).unwrap()
}

/// Evaluate the pairwise representation of the dispersion energy (failable)
pub fn get_pairwise_dispersion_f(
    structure: &DFTD4Structure,
    model: &DFTD4Model,
    param: &DFTD4Param,
) -> Result<(Vec<f64>, Vec<f64>), DFTD4Error> {
    let natoms = structure.get_natoms();
    let mut pair_energy2 = vec![0.0; natoms * natoms];
    let mut pair_energy3 = vec![0.0; natoms * natoms];
    let mut error = DFTD4Error::new();

    unsafe {
        ffi::dftd4_get_pairwise_dispersion(
            error.get_c_ptr(),
            structure.ptr,
            model.ptr,
            param.ptr,
            pair_energy2.as_mut_ptr(),
            pair_energy3.as_mut_ptr(),
        )
    };
    match error.check() {
        true => Err(error),
        false => Ok((pair_energy2, pair_energy3)),
    }
}

/// Evaluate the pairwise representation of the dispersion energy
pub fn get_pairwise_dispersion(
    structure: &DFTD4Structure,
    model: &DFTD4Model,
    param: &DFTD4Param,
) -> (Vec<f64>, Vec<f64>) {
    get_pairwise_dispersion_f(structure, model, param).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_version() {
        println!("API version: {}", get_api_version());
    }

    #[test]
    fn test_get_api_version_compact() {
        println!("API version: {:?}", get_api_version_compact());
    }
}
