use personnel::{AstronautJob, Candidate};

fn astronaut_job_score(job: &AstronautJob) -> u32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn job_score(candidate: &Candidate) -> u32 {
    match candidate.secondary_job {
        None => astronaut_job_score(&candidate.primary_job).pow(2) % 577,
        _ => {
            astronaut_job_score(&candidate.primary_job)
                * astronaut_job_score(candidate.secondary_job.as_ref().unwrap())
                % 577
        }
    }
}

fn candidate_score(candidate: &Candidate) -> u32 {
    (job_score(candidate) + u32::from(candidate.health)) * u32::from(candidate.age) % 3929
}

fn sort_candidates(candidates: &mut [Candidate]) {
    candidates.sort_by_key(candidate_score);
}

fn main() {
    let mut candidates = Candidate::load_candidate_file();
    sort_candidates(&mut candidates);
}

#[test]
fn test_astronaut_job_score() {
    assert_eq!(astronaut_job_score(&AstronautJob::Biogeochemist), 251);
    assert_eq!(astronaut_job_score(&AstronautJob::Biologist), 257);
    assert_eq!(astronaut_job_score(&AstronautJob::Engineer), 263);
    assert_eq!(astronaut_job_score(&AstronautJob::Geologist), 269);
    assert_eq!(astronaut_job_score(&AstronautJob::Mechanic), 271);
    assert_eq!(astronaut_job_score(&AstronautJob::Medic), 277);
    assert_eq!(astronaut_job_score(&AstronautJob::RoverOp), 281);
    assert_eq!(astronaut_job_score(&AstronautJob::Scientist), 283);
}

#[test]
fn test_job_score() {
    let candidate_two_jobs = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: Some(AstronautJob::Biologist),
        health: 0,
        age: 0,
    };
    let candidate_one_job = Candidate {
        primary_job: AstronautJob::Mechanic,
        secondary_job: None,
        health: 0,
        age: 0,
    };

    assert_eq!(job_score(&candidate_two_jobs), 92);
    assert_eq!(job_score(&candidate_one_job), 162);
}

#[test]
fn test_candidate_score() {
    let c1 = Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: Some(AstronautJob::Biologist),
        health: 20,
        age: 29,
    };
    let c2 = Candidate {
        primary_job: AstronautJob::Mechanic,
        secondary_job: None,
        health: 19,
        age: 34,
    };

    assert_eq!(candidate_score(&c1), 3248);
    assert_eq!(candidate_score(&c2), 2225);
}

#[test]
fn test_candidate_ordering() {
    let mut candidates = Vec::<Candidate>::new();

    candidates.push(Candidate {
        primary_job: AstronautJob::RoverOp,
        secondary_job: Some(AstronautJob::Biologist),
        health: 20,
        age: 29,
    });

    candidates.push(Candidate {
        primary_job: AstronautJob::Mechanic,
        secondary_job: None,
        health: 19,
        age: 34,
    });

    assert_eq!(candidate_score(&candidates[0]), 3248);
    assert_eq!(candidate_score(&candidates[1]), 2225);

    sort_candidates(&mut candidates);

    assert_eq!(candidate_score(&candidates[0]), 2225);
    assert_eq!(candidate_score(&candidates[1]), 3248);
}
