mod tilts;
mod smashes;
mod aerials;
mod attackdash;
mod attackjab;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    attackdash::install(agent);
    attackjab::install(agent);
}