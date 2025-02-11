
use chrono::{NaiveDate, Datelike, Duration};

pub struct Fecha {
    pub day: u8,
    pub month: u8,
    pub year: i32,
}

impl Fecha {
    pub fn new (day: u8, month: u8, year: i32) -> Self {
        Self {day, month, year}
    }

    pub fn es_bisiesto (&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    pub fn es_fecha_valida (&self) -> bool {
        if self.year < 0 || self.month < 1 || self.month > 12 || self.day < 1 || self.day > 31{
            return false;
        } 
        if (self.month == 4 || self.month == 6 || self.month == 9 || self.month == 11) && self.day == 31{
            return false;
        }    
        if self.month == 2 {
            if self.day == 30 || self.day == 31 {
                return false;
            }   
            if !self.es_bisiesto() && self.day == 29 {
                return false;
            }                 
        }
        true
    }

    // Método `sumar_dias`: suma días a la fecha
    pub fn sumar_dias(&mut self, dias: i64) {
        if let Some(fecha) = NaiveDate::from_ymd_opt(self.year, self.month as u32, self.day as u32) {
            let nueva_fecha = fecha + Duration::days(dias);
            self.year = nueva_fecha.year();
            self.month = nueva_fecha.month() as u8;
            self.day = nueva_fecha.day() as u8;
        }
    }

    // Método `restar_dias`: resta días a la fecha
    pub fn restar_dias(&mut self, dias: i64) {
        if let Some(fecha) = NaiveDate::from_ymd_opt(self.year, self.month as u32, self.day as u32) {
            let nueva_fecha = fecha - Duration::days(dias);
            self.year = nueva_fecha.year();
            self.month = nueva_fecha.month() as u8;
            self.day = nueva_fecha.day() as u8;
        }
    }

    // Método `es_mayor`: compara dos fechas
    pub fn es_mayor(&self, otra: &Fecha) -> bool {
        if let (Some(fecha1), Some(fecha2)) = (
            NaiveDate::from_ymd_opt(self.year, self.month as u32, self.day as u32),
            NaiveDate::from_ymd_opt(otra.year, otra.month as u32, otra.day as u32),
        ) {
            fecha1 > fecha2
        } else {
            false // Retorna false si alguna de las fechas no es válida
        }
    }
}