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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_creation_with_dimensions() {
        // Test 2D creation
        let coords_2d = Coordinates::new_2d(10.0, 20.0);
        assert_eq!(coords_2d.dimensions(), 2);
        assert_eq!(coords_2d.get(0), Some(10.0));
        assert_eq!(coords_2d.get(1), Some(20.0));
        
        // Test 4D creation
        let coords_4d = Coordinates::new_4d(1.0, 2.0, 3.0, 4.0);
        assert_eq!(coords_4d.dimensions(), 4);
        assert_eq!(coords_4d.get(0), Some(1.0));
        assert_eq!(coords_4d.get(1), Some(2.0));
        assert_eq!(coords_4d.get(2), Some(3.0));
        assert_eq!(coords_4d.get(3), Some(4.0));
    }
    
    #[test]
    fn test_coordinate_set_get() {
        let mut coords = Coordinates::new(3);
        
        // Set values and check them
        assert_eq!(coords.set(0, 10.0), true);
        assert_eq!(coords.set(1, 20.0), true);
        assert_eq!(coords.set(2, 30.0), true);
        
        assert_eq!(coords.get(0), Some(10.0));
        assert_eq!(coords.get(1), Some(20.0));
        assert_eq!(coords.get(2), Some(30.0));
        
        // Out of bounds access should return None or false
        assert_eq!(coords.get(3), None);
        assert_eq!(coords.set(3, 40.0), false);
    }
    
    #[test]
    fn test_coordinate_labels() {
        // Test setting labels during creation
        let coords = Coordinates::new(4).with_labels(vec!["x", "y", "z", "time"]);
        
        // Test get/set by label
        assert_eq!(coords.get_by_label("x"), Some(0.0));
        assert_eq!(coords.get_by_label("y"), Some(0.0));
        assert_eq!(coords.get_by_label("z"), Some(0.0));
        assert_eq!(coords.get_by_label("time"), Some(0.0));
        
        // Test non-existent label
        assert_eq!(coords.get_by_label("w"), None);
        
        // Test set by label
        let mut coords2 = coords.clone();
        assert_eq!(coords2.set_by_label("x", 100.0), true);
        assert_eq!(coords2.set_by_label("y", 200.0), true);
        assert_eq!(coords2.set_by_label("z", 300.0), true);
        assert_eq!(coords2.set_by_label("time", 400.0), true);
        
        assert_eq!(coords2.get_by_label("x"), Some(100.0));
        assert_eq!(coords2.get_by_label("y"), Some(200.0));
        assert_eq!(coords2.get_by_label("z"), Some(300.0));
        assert_eq!(coords2.get_by_label("time"), Some(400.0));
        
        // Non-existent label should return false
        assert_eq!(coords2.set_by_label("w", 500.0), false);
    }
    
    #[test]
    fn test_distance_calculation() {
        // Test 2D distance (Pythagorean)
        let coords1 = Coordinates::new_2d(0.0, 0.0);
        let coords2 = Coordinates::new_2d(3.0, 4.0);
        assert_eq!(coords1.distance(&coords2), 5.0);
        
        // Test 3D distance
        let coords3 = Coordinates::new_3d(0.0, 0.0, 0.0);
        let coords4 = Coordinates::new_3d(2.0, 3.0, 6.0);
        assert_eq!(coords3.distance(&coords4), 7.0);
        
        // Different dimensions should return NaN
        let coords5 = Coordinates::new_2d(0.0, 0.0);
        let coords6 = Coordinates::new_3d(1.0, 1.0, 1.0);
        assert!(coords5.distance(&coords6).is_nan());
    }
    
    #[test]
    fn test_direction_vector() {
        // Test horizontal direction (1, 0)
        let start = Coordinates::new_2d(0.0, 0.0);
        let target = Coordinates::new_2d(5.0, 0.0);
        let direction = start.direction_to(&target).unwrap();
        assert_eq!(direction.get(0), Some(1.0));
        assert_eq!(direction.get(1), Some(0.0));
        
        // Test vertical direction (0, 1)
        let start = Coordinates::new_2d(0.0, 0.0);
        let target = Coordinates::new_2d(0.0, 5.0);
        let direction = start.direction_to(&target).unwrap();
        assert_eq!(direction.get(0), Some(0.0));
        assert_eq!(direction.get(1), Some(1.0));
        
        // Test diagonal (normalized)
        let start = Coordinates::new_2d(0.0, 0.0);
        let target = Coordinates::new_2d(3.0, 4.0);
        let direction = start.direction_to(&target).unwrap();
        assert!((direction.get(0).unwrap() - 0.6).abs() < 0.0001); // ~3/5
        assert!((direction.get(1).unwrap() - 0.8).abs() < 0.0001); // ~4/5
    }
    
    #[test]
    fn test_to_string() {
        // Test default display
        let coords = Coordinates::new_2d(1.5, 2.75);
        assert_eq!(coords.to_string(), "(x:1.5, y:2.75)");
        
        // Test custom labels
        let coords = Coordinates::new(2).with_labels(vec!["horizontal", "vertical"]);
        let mut custom_coords = coords.clone();
        custom_coords.set(0, 10.5);
        custom_coords.set(1, 20.25);
        assert_eq!(custom_coords.to_string(), "(horizontal:10.5, vertical:20.25)");
    }
    
    #[test]
    fn test_move_toward() {
        // Test moving halfway to target
        let mut coords = Coordinates::new_2d(0.0, 0.0);
        let target = Coordinates::new_2d(10.0, 10.0);
        
        assert_eq!(coords.move_toward(&target, 5.0 * 2.0_f32.sqrt()), true);
        assert!((coords.get(0).unwrap() - 5.0).abs() < 0.0001);
        assert!((coords.get(1).unwrap() - 5.0).abs() < 0.0001);
        
        // Test moving to target
        coords.move_toward(&target, 5.0 * 2.0_f32.sqrt());
        assert!((coords.get(0).unwrap() - 10.0).abs() < 0.0001);
        assert!((coords.get(1).unwrap() - 10.0).abs() < 0.0001);
    }
} 