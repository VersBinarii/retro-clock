use crate::clock::Time;
use embedded_hal::digital::v2::OutputPin;

pub struct NibbleWriter<D0, D1, D2, D3> {
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
}

impl<D0, D1, D2, D3> NibbleWriter<D0, D1, D2, D3>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
{
    pub fn new(d0: D0, d1: D1, d2: D2, d3: D3) -> Self {
        Self { d0, d1, d2, d3 }
    }
    pub fn write_nibble(&mut self, data: u8) {
        if data & 0b0000_0001 != 0 {
            let _ = self.d0.set_high();
        } else {
            let _ = self.d0.set_low();
        }
        if data & 0b0000_0010 != 0 {
            let _ = self.d1.set_high();
        } else {
            let _ = self.d1.set_low();
        }
        if data & 0b0000_0100 != 0 {
            let _ = self.d2.set_high();
        } else {
            let _ = self.d2.set_low();
        }
        if data & 0b0000_1000 != 0 {
            let _ = self.d3.set_high();
        } else {
            let _ = self.d3.set_low();
        }
    }
}

pub struct NixieDriver<
    HT0,
    HT1,
    HT2,
    HT3,
    HU0,
    HU1,
    HU2,
    HU3,
    MT0,
    MT1,
    MT2,
    MT3,
    MU0,
    MU1,
    MU2,
    MU3,
> {
    hour_t: NibbleWriter<HT0, HT1, HT2, HT3>,
    hour_u: NibbleWriter<HU0, HU1, HU2, HU3>,
    min_t: NibbleWriter<MT0, MT1, MT2, MT3>,
    min_u: NibbleWriter<MU0, MU1, MU2, MU3>,
}

impl<HT0, HT1, HT2, HT3, HU0, HU1, HU2, HU3, MT0, MT1, MT2, MT3, MU0, MU1, MU2, MU3>
    NixieDriver<HT0, HT1, HT2, HT3, HU0, HU1, HU2, HU3, MT0, MT1, MT2, MT3, MU0, MU1, MU2, MU3>
where
    HT0: OutputPin,
    HT1: OutputPin,
    HT2: OutputPin,
    HT3: OutputPin,
    HU0: OutputPin,
    HU1: OutputPin,
    HU2: OutputPin,
    HU3: OutputPin,
    MT0: OutputPin,
    MT1: OutputPin,
    MT2: OutputPin,
    MT3: OutputPin,
    MU0: OutputPin,
    MU1: OutputPin,
    MU2: OutputPin,
    MU3: OutputPin,
{
    pub fn new(
        hour_t: NibbleWriter<HT0, HT1, HT2, HT3>,
        hour_u: NibbleWriter<HU0, HU1, HU2, HU3>,
        min_t: NibbleWriter<MT0, MT1, MT2, MT3>,
        min_u: NibbleWriter<MU0, MU1, MU2, MU3>,
    ) -> Self {
        Self {
            hour_t,
            hour_u,
            min_t,
            min_u,
        }
    }

    pub fn show(&mut self, time: &Time) {
        let hours = time.hours;
        let minutes = time.minutes;

        let hour_t = hours / 10;
        let hour_u = hours % 10;

        let min_t = minutes / 10;
        let min_u = minutes % 10;

        self.hour_t.write_nibble(hour_t);
        self.hour_u.write_nibble(hour_u);
        self.min_t.write_nibble(min_t);
        self.min_u.write_nibble(min_u);
    }
}
