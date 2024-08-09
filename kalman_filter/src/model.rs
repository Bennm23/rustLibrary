use core::fmt;

use nalgebra::{SMatrix, SVector};
use rand_distr::num_traits::{zero, ToPrimitive, Zero};


pub struct IModel<const STATES : usize> {

    pub x : SVector<f64, STATES>,
    pub q : SMatrix<f64, STATES, STATES>,
    pub p : SMatrix<f64, STATES, STATES>,
}

impl<const STATES : usize> IModel<STATES> {
    pub fn new() -> Self {
        Self {
            x : SVector::<f64, STATES>::zeros(),
            q : SMatrix::<f64, STATES, STATES>::zeros(),
            p : SMatrix::<f64, STATES, STATES>::zeros(),
        }
    }

    pub fn init_q(&mut self, q_scalar : f64) {

        for i in 0 .. STATES {
            self.q[(i, i)] = q_scalar;
        };
    }

    pub fn f(&self, dt : f64) -> SMatrix<f64, STATES, STATES> {
        let mut res : SMatrix<f64, STATES, STATES> = SMatrix::<f64, STATES, STATES>::zeros();
        for i in 0 .. STATES {

            res[(i,i)] = 1.0;

            if i < STATES / 2 {
                res[(i, i + (STATES/2))] = dt;
            }
        };

        println!("F MATR = {res}");

        res
    }
}

impl<const STATES : usize> fmt::Display for IModel<STATES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STATE = {}ERR COVARIANCE = {}", self.x, self.p)
    }
}

pub struct IMeas<const DIM : usize> {

    pub r : SMatrix<f64, DIM, DIM>, //Measurement Noise Covariance
    pub z : SVector<f64, DIM>, //Measurement
}

impl<const DIM : usize> IMeas<DIM> {
    pub fn new(_r : SMatrix<f64, DIM, DIM>, _z : SVector<f64, DIM>) -> Self {
        Self {
            r : _r,
            z : _z,
        }
    }

    pub fn build_h<const state_dim : usize>(&self) -> SMatrix<f64, DIM, state_dim> {

        let mut h : SMatrix<f64, DIM, state_dim> = SMatrix::<f64, DIM,state_dim>::zeros();
        for i in 0 .. DIM {
            h[(i, i)] = 1.0;
        }
        h
    }
}

impl<const DIM : usize> fmt::Display for IMeas<DIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MEAS = {}", self.z)
    }
}