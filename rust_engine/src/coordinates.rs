use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use serde::{Serialize, Deserialize};

/// A flexible coordinate system that can represent positions in any number of dimensions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    /// The values for each dimension
    pub values: Vec<f32>,
    /// Optional labels for each dimension (e.g., "x", "y", "z", "time", etc.)
    pub labels: Option<Vec<String>>,
}

impl Coordinates {
    /// Create a new coordinates instance with the specified number of dimensions, all initialized to 0.0
    pub fn new(dimensions: usize) -> Self {
        Coordinates {
            values: vec![0.0; dimensions],
            labels: None,
        }
    }
    
    /// Create coordinates with the given values
    pub fn from_values(values: Vec<f32>) -> Self {
        Coordinates {
            values,
            labels: None,
        }
    }
    
    /// Create 1D coordinates
    pub fn new_1d(x: f32) -> Self {
        Coordinates {
            values: vec![x],
            labels: Some(vec!["x".to_string()]),
        }
    }
    
    /// Create 2D coordinates
    pub fn new_2d(x: f32, y: f32) -> Self {
        Coordinates {
            values: vec![x, y],
            labels: Some(vec!["x".to_string(), "y".to_string()]),
        }
    }
    
    /// Create 3D coordinates
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Coordinates {
            values: vec![x, y, z],
            labels: Some(vec!["x".to_string(), "y".to_string(), "z".to_string()]),
        }
    }
    
    /// Create 4D coordinates (including time)
    pub fn new_4d(x: f32, y: f32, z: f32, t: f32) -> Self {
        Coordinates {
            values: vec![x, y, z, t],
            labels: Some(vec!["x".to_string(), "y".to_string(), "z".to_string(), "t".to_string()]),
        }
    }
    
    /// Create coordinates with custom dimension labels
    pub fn with_labels(mut self, labels: Vec<&str>) -> Self {
        if labels.len() == self.values.len() {
            self.labels = Some(labels.iter().map(|&s| s.to_string()).collect());
        }
        self
    }
    
    /// Get the number of dimensions
    pub fn dimensions(&self) -> usize {
        self.values.len()
    }
    
    /// Get a value for a specific dimension by index
    pub fn get(&self, dimension: usize) -> Option<f32> {
        self.values.get(dimension).copied()
    }
    
    /// Set a value for a specific dimension by index
    pub fn set(&mut self, dimension: usize, value: f32) -> bool {
        if dimension < self.values.len() {
            self.values[dimension] = value;
            true
        } else {
            false
        }
    }
    
    /// Get a value for a dimension by label
    pub fn get_by_label(&self, label: &str) -> Option<f32> {
        if let Some(labels) = &self.labels {
            for (i, l) in labels.iter().enumerate() {
                if l == label {
                    return self.values.get(i).copied();
                }
            }
        }
        None
    }
    
    /// Set a value for a dimension by label
    pub fn set_by_label(&mut self, label: &str, value: f32) -> bool {
        if let Some(labels) = &self.labels {
            for (i, l) in labels.iter().enumerate() {
                if l == label && i < self.values.len() {
                    self.values[i] = value;
                    return true;
                }
            }
        }
        false
    }
    
    /// Calculate the distance between two sets of coordinates
    pub fn distance(&self, other: &Coordinates) -> f32 {
        if self.dimensions() != other.dimensions() {
            return f32::NAN; // Not comparable
        }
        
        let mut sum_of_squares = 0.0;
        for i in 0..self.dimensions() {
            let diff = self.values[i] - other.values[i];
            sum_of_squares += diff * diff;
        }
        
        sum_of_squares.sqrt()
    }
    
    /// Get a normalized vector pointing from these coordinates to the target
    pub fn direction_to(&self, target: &Coordinates) -> Option<Coordinates> {
        if self.dimensions() != target.dimensions() {
            return None; // Not comparable
        }
        
        let distance = self.distance(target);
        if distance == 0.0 || distance.is_nan() {
            return None;
        }
        
        let mut direction = vec![0.0; self.dimensions()];
        for i in 0..self.dimensions() {
            direction[i] = (target.values[i] - self.values[i]) / distance;
        }
        
        Some(Coordinates {
            values: direction,
            labels: self.labels.clone(),
        })
    }
    
    /// Move these coordinates toward a target by a certain amount
    pub fn move_toward(&mut self, target: &Coordinates, distance: f32) -> bool {
        if let Some(direction) = self.direction_to(target) {
            for i in 0..self.dimensions() {
                self.values[i] += direction.values[i] * distance;
            }
            true
        } else {
            false
        }
    }
    
    /// Convert to simple 2D coordinates for backward compatibility
    pub fn to_2d(&self) -> (f32, f32) {
        let x = self.get(0).unwrap_or(0.0);
        let y = self.get(1).unwrap_or(0.0);
        (x, y)
    }
    
    /// Convert to simple 3D coordinates
    pub fn to_3d(&self) -> (f32, f32, f32) {
        let x = self.get(0).unwrap_or(0.0);
        let y = self.get(1).unwrap_or(0.0);
        let z = self.get(2).unwrap_or(0.0);
        (x, y, z)
    }
}

// Addition operation
impl Add for Coordinates {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        if self.dimensions() != other.dimensions() {
            return self; // Can't add different dimensions
        }
        
        let mut result = self.clone();
        for i in 0..self.dimensions() {
            result.values[i] += other.values[i];
        }
        result
    }
}

// Subtraction operation
impl Sub for Coordinates {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        if self.dimensions() != other.dimensions() {
            return self; // Can't subtract different dimensions
        }
        
        let mut result = self.clone();
        for i in 0..self.dimensions() {
            result.values[i] -= other.values[i];
        }
        result
    }
}

// Scalar multiplication
impl Mul<f32> for Coordinates {
    type Output = Self;
    
    fn mul(self, scalar: f32) -> Self {
        let mut result = self.clone();
        for i in 0..self.dimensions() {
            result.values[i] *= scalar;
        }
        result
    }
}

// Scalar division
impl Div<f32> for Coordinates {
    type Output = Self;
    
    fn div(self, scalar: f32) -> Self {
        if scalar == 0.0 {
            return self; // Avoid division by zero
        }
        
        let mut result = self.clone();
        for i in 0..self.dimensions() {
            result.values[i] /= scalar;
        }
        result
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        
        if let Some(labels) = &self.labels {
            for (i, (value, label)) in self.values.iter().zip(labels.iter()).enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", label, value)?;
            }
        } else {
            for (i, value) in self.values.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", value)?;
            }
        }
        
        write!(f, ")")
    }
} 