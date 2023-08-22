use anchor_lang::prelude::*;

use crate::constants::*;

#[derive(Debug, Default)]
#[account]
pub struct VolatilityAccumulator {
    pub bump: u8,
    pub market: Pubkey,
    pub creator: Pubkey,
    pub buffer: [f64; VOLATILITY_BUFFER_SIZE],
}

impl VolatilityAccumulator {
    pub const LEN: usize = 8 + (1 * 1) + (VOLATILITY_BUFFER_SIZE * 8) + (2 * 32);

    pub fn update_buffer(&mut self, new_val: f64) {
        for i in 1..self.buffer.len() {
            self.buffer[i - 1] = self.buffer[i];
        }

        self.buffer[self.buffer.len() - 1] = new_val;
    }

    pub fn get_volatility(&mut self, use_log_returns: bool) -> f64 {
        if use_log_returns {
            let mut sum_squared_log_returns: f64 = 0.0;

            for i in 1..self.buffer.len() {
                sum_squared_log_returns += (self.buffer[i] / self.buffer[i - 1]).ln().powf(2.0);
            }

            let avg_returns = sum_squared_log_returns / self.buffer.len() as f64;

            avg_returns.sqrt()
        }
        else {
            let mut sum_squared_returns: f64 = 0.0;

            for i in 1..self.buffer.len() {
                sum_squared_returns += (self.buffer[i] - self.buffer[i - 1]).powf(2.0);
            }

            let avg_returns = sum_squared_returns / self.buffer.len() as f64;

            avg_returns.sqrt()
        }
    }
}
