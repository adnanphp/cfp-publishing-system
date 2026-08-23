use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::{
    models::{PlagiarismCase, Vote, Text, Comment},
    enums::{PlagiarismStatus, PlagiarismResolution, VoteType},
    events::plagiarism_events::PlagiarismEvent,
};

#[derive(Debug, Clone)]
pub struct PlagiarismAggregate {
    pub case: PlagiarismCase,
    pub votes: Vec<Vote>,
    pub comments: Vec<Comment>,
    pub text: Option<Text>,
    pub total_votes_required: i32,
    pub pending_events: Vec<PlagiarismEvent>,
}

impl PlagiarismAggregate {
    pub fn new(
        committee_id: Uuid,
        text_id: Uuid,
        description: String,
        total_votes_required: i32,
    ) -> Self {
        let case = PlagiarismCase::new(
            committee_id,
            text_id,
            description,
        );

        Self {
            case,
            votes: Vec::new(),
            comments: Vec::new(),
            text: None,
            total_votes_required,
            pending_events: Vec::new(),
        }
    }

    pub fn case_id(&self) -> Uuid {
        self.case.case_id
    }

    pub fn start_review(&mut self) {
        if self.case.status == PlagiarismStatus::Open {
            self.case.status = PlagiarismStatus::UnderReview;
            
            self.pending_events.push(PlagiarismEvent::ReviewStarted {
                case_id: self.case_id(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn start_voting(&mut self) {
        if self.case.status == PlagiarismStatus::UnderReview {
            self.case.status = PlagiarismStatus::Voting;
            
            self.pending_events.push(PlagiarismEvent::VotingStarted {
                case_id: self.case_id(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn cast_vote(&mut self, vote: Vote) -> bool {
        // Check if member already voted
        if self.votes.iter().any(|v| v.member_id == vote.member_id) {
            return false;
        }

        // Check if voting is open
        if self.case.status != PlagiarismStatus::Voting {
            return false;
        }

        self.votes.push(vote);
        
        self.pending_events.push(PlagiarismEvent::VoteCast {
            case_id: self.case_id(),
            member_id: vote.member_id,
            vote_type: vote.vote,
            timestamp: Utc::now(),
        });

        // Check if voting is complete
        if self.votes.len() as i32 >= self.total_votes_required {
            self.resolve_case();
        }

        true
    }

    pub fn resolve_case(&mut self) {
        let plagiarized_votes = self.votes.iter()
            .filter(|v| v.is_plagiarized_vote())
            .count();
        
        let not_plagiarized_votes = self.votes.iter()
            .filter(|v| v.is_not_plagiarized_vote())
            .count();

        let total_votes_cast = self.votes.len();
        let required_majority = (total_votes_cast as f64 * 2.0 / 3.0).ceil() as usize;

        let resolution = if plagiarized_votes >= required_majority {
            PlagiarismResolution::Plagiarized
        } else if not_plagiarized_votes > plagiarized_votes {
            PlagiarismResolution::NotPlagiarized
        } else {
            // Tie or insufficient votes
            PlagiarismResolution::Dismissed
        };

        self.case.resolution = Some(resolution);
        self.case.status = PlagiarismStatus::Closed;
        self.case.closed_date = Some(Utc::now());

        self.pending_events.push(PlagiarismEvent::CaseResolved {
            case_id: self.case_id(),
            resolution,
            timestamp: Utc::now(),
        });

        if resolution == PlagiarismResolution::Plagiarized {
            self.pending_events.push(PlagiarismEvent::PlagiarismConfirmed {
                case_id: self.case_id(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn appeal_case(&mut self, reason: String) {
        if self.case.status == PlagiarismStatus::Closed {
            self.case.status = PlagiarismStatus::Appealed;
            self.case.resolution = Some(PlagiarismResolution::Appealed);
            
            self.pending_events.push(PlagiarismEvent::CaseAppealed {
                case_id: self.case_id(),
                reason,
                timestamp: Utc::now(),
            });
        }
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    pub fn get_vote_count(&self, vote_type: VoteType) -> usize {
        self.votes.iter()
            .filter(|v| v.vote == vote_type)
            .count()
    }

    pub fn get_voting_progress(&self) -> (usize, i32, f32) {
        let votes_cast = self.votes.len();
        let progress_percentage = (votes_cast as f32 / self.total_votes_required as f32) * 100.0;
        (votes_cast, self.total_votes_required, progress_percentage)
    }

    pub fn has_member_voted(&self, member_id: Uuid) -> bool {
        self.votes.iter().any(|v| v.member_id == member_id)
    }

    pub fn get_member_vote(&self, member_id: Uuid) -> Option<&Vote> {
        self.votes.iter().find(|v| v.member_id == member_id)
    }

    pub fn is_voting_open(&self) -> bool {
        self.case.status == PlagiarismStatus::Voting
    }

    pub fn is_case_closed(&self) -> bool {
        self.case.status == PlagiarismStatus::Closed
    }

    pub fn is_case_appealed(&self) -> bool {
        self.case.status == PlagiarismStatus::Appealed
    }

    pub fn requires_two_thirds_majority(&self) -> bool {
        true // Plagiarism cases always require 2/3 majority
    }

    pub fn take_events(&mut self) -> Vec<PlagiarismEvent> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn apply_event(&mut self, event: PlagiarismEvent) {
        match event {
            PlagiarismEvent::CaseOpened { .. } => {}
            PlagiarismEvent::ReviewStarted { .. } => {
                self.case.status = PlagiarismStatus::UnderReview;
            }
            PlagiarismEvent::VotingStarted { .. } => {
                self.case.status = PlagiarismStatus::Voting;
            }
            PlagiarismEvent::CaseResolved { resolution, .. } => {
                self.case.status = PlagiarismStatus::Closed;
                self.case.resolution = Some(resolution);
                self.case.closed_date = Some(Utc::now());
            }
            PlagiarismEvent::CaseAppealed { reason, .. } => {
                self.case.status = PlagiarismStatus::Appealed;
                self.case.resolution = Some(PlagiarismResolution::Appealed);
            }
            PlagiarismEvent::VoteCast { member_id, vote_type, .. } => {
                // Vote already added in cast_vote method
            }
            PlagiarismEvent::PlagiarismConfirmed { .. } => {
                // Handle plagiarism confirmation logic
            }
        }
    }
}
