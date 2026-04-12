mod attacks3; //ftilt
mod attackhi3;
mod attacklw3;

pub fn install(agent: &mut smashline::Agent) {
    attacks3::install(agent);
    attackhi3::install(agent);
    attacklw3::install(agent);
}