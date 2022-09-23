enum Autos{
    Pkw,
    Lkw,
}

struct PKW{
    ps: u16
}

impl PKW{
    fn new(_ps: u16) -> PKW{
        PKW{ ps: _ps}
    }
}

struct LKW{
    ps: u16
}

impl LKW{
    fn new(_ps: u16) -> LKW{
        LKW{ ps: _ps}
    }
}

trait Auto {
    fn say_who_am_i(&self);

    fn get_ps(&self) -> u16;
}

impl Auto for PKW{
    fn say_who_am_i(&self){
        println!("{}", "Ich bin ein PKW!")
    }

    fn get_ps(&self) -> u16 {
        self.ps
    }
}

impl Auto for LKW{
    fn say_who_am_i(&self){
        println!("{}", "Ich bin ein LKW!")
    }

    fn get_ps(&self) -> u16 {
        self.ps
    }
}

struct Autofabrik{
}

impl Autofabrik{
    fn new(autos: Autos, _ps: u16) -> Box<dyn Auto>{
        match autos{
            Autos::Lkw => Box::new(LKW::new(_ps)),
            Autos::Pkw => Box::new(PKW::new(_ps)),
        }
    }
}

struct Werkstatt{
    fahrzeug : Box<dyn Auto>
}

impl Werkstatt{
    fn new(auto: Box<dyn Auto>) -> Werkstatt{
        return Werkstatt{ fahrzeug : auto};
   }

    fn zeige_fahrzeug(&self){
        print!("Ich habe {0} PS: ", self.fahrzeug.get_ps());

        self.fahrzeug.say_who_am_i();
    }
}



fn main() {
    let auto1 = Autofabrik::new(Autos::Lkw, 300);

    let auto2 = Autofabrik::new(Autos::Pkw, 80);

    let werkstatt: Werkstatt = Werkstatt::new(auto1);

    werkstatt.zeige_fahrzeug();

    let werkstatt: Werkstatt = Werkstatt::new(auto2);

    werkstatt.zeige_fahrzeug();
}
