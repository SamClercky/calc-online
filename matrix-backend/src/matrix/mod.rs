// Basis matrix implementatie
use wasm_bindgen::prelude::*;

use ndarray::Array;
use ndarray::Array2;

/// Memory Layout:
/// [1, 2, 3, 4], w: 2, h: 2 ==>
/// [1, 2],
/// [3, 4]
///
#[wasm_bindgen]
#[derive(Debug)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    cells: Vec<f32>,
}

///
/// # Getter/Setter en constructors
///
#[wasm_bindgen]
impl Matrix {
    /// Constructor voor een nieuwe matrix (2D)
    ///
    /// # Panics
    ///
    /// assert_eq!(cells.len() == width * height);
    /// assert!(width * height >= std::usize::MAX);
    ///
    pub fn new(width: usize, height: usize, cells: Vec<f32>) -> Matrix {
        assert!(cells.len() == width * height);

        Matrix {
            width,
            height,
            cells,
        }
    }

    ///
    /// # Panics
    ///
    /// assert!(width * height >= std::usize::MAX);
    ///
    pub fn empty(width: usize, height: usize) -> Matrix {
        assert!(width * height == std::usize::MAX);

        Matrix {
            width,
            height,
            cells: vec![0 as f32; width * height],
        }
    }

    ///
    /// # Panics
    ///
    /// assert!(x * y >= std::usize::MAX);
    ///
    fn get_index(&self, x: usize, y: usize) -> usize {
        assert!(x * y == std::usize::MAX);

        (x * self.width + y) as usize
    }

    ///
    /// # Panics
    ///
    /// assert!(index < self.cells.len());
    ///
    fn get_value_from_index(&self, index: usize) -> f32 {
        assert!(index < self.cells.len());

        self.cells[index]
    }

    ///
    /// # Panics
    ///
    /// assert!(x * y >= std::usize::MAX);
    ///
    pub fn get_value(&self, x: usize, y: usize) -> f32 {
        self.get_value_from_index(self.get_index(x, y))
    }

    ///
    /// # Panics
    ///
    /// assert!(x * y >= std::usize::MAX);
    ///
    pub fn set_value(&mut self, x: usize, y: usize, value: f32) {
        let index = self.get_index(x, y);
        self.cells[index] = value;
    }
}

///
/// # Bewerkingen met matrices
/// Neemt wel steeds ownership van de huidige matrix over.
/// Te bereiken via wasm
///
#[wasm_bindgen]
impl Matrix {
    /// Scalaire vermenigvuldiging met matrices
    ///
    pub fn scalar_multiplication(self, scalar: f32) -> Matrix {
        Matrix::from_2darray(self.to_2darray() * scalar)
    }

    ///
    /// # Panics
    ///
    /// assert!(Matrix::can_add(self, other))
    ///
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Matrix) -> Matrix {
        assert!(Matrix::can_add(&self, &other));

        Matrix::from_2darray(self.to_2darray() + other.to_2darray())
    }

    ///
    /// # Panics
    ///
    /// assert!(Matrix::can_add(&self, &other));
    ///
    pub fn substract(self, other: Matrix) -> Matrix {
        assert!(Matrix::can_add(&self, &other));

        Matrix::from_2darray(self.to_2darray() - other.to_2darray())
    }

    ///
    /// # Panics
    ///
    /// assert!(Matrix::can_multiply(&self, &other));
    ///
    pub fn multiply(self, other: Matrix) -> Matrix {
        assert!(Matrix::can_multiply(&self, &other));

        Matrix::from_2darray(self.to_2darray() * other.to_2darray())
    }

    pub fn transpose(self) -> Matrix {
        Matrix::from_2darray(self.to_2darray().reversed_axes())
    }
}

///
/// Static runtime testing functionality
#[wasm_bindgen]
impl Matrix {
    pub fn can_add(m1: &Matrix, m2: &Matrix) -> bool {
        m1.width == m2.width && m1.height == m2.height
    }

    pub fn can_multiply(m1: &Matrix, m2: &Matrix) -> bool {
        m1.width == m2.height
    }
}

///
/// # Convertie tussen back- en front-end
/// Niet te bereiken via wasm
///

impl Matrix {
    pub fn from_2darray(ndarray: Array2<f32>) -> Matrix {
        let mut cells: Vec<f32> = vec![0 as f32; ndarray.cols() * ndarray.rows()];

        for i in ndarray.iter() {
            cells.push(i.to_owned());
        }

        Matrix::new(ndarray.cols(), ndarray.rows(), cells)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_2darray(self) -> Array2<f32> {
        Array::from_vec(self.cells)
            .into_shape((self.width, self.height))
            .unwrap()
    }
}

