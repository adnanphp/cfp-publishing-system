use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::{
    models::{Member, Author, Admin, Moderator, Message, Notification, Download},
    enums::{MemberStatus, RoleEnum},
    value_objects::{PhoneNumber, Email, Address, VerificationMatrix},
    events::member_events::MemberEvent,
};

#[derive(Debug, Clone)]
pub struct MemberAggregate {
    pub member: Member,
    pub author_profile: Option<Author>,
    pub admin_profile: Option<Admin>,
    pub moderator_profile: Option<Moderator>,
    pub unread_messages: Vec<Message>,
    pub unread_notifications: Vec<Notification>,
    pub recent_downloads: Vec<Download>,
    pub pending_events: Vec<MemberEvent>,
}

impl MemberAggregate {
    pub fn new(
        name: String,
        organization: String,
        pseudonym: Option<String>,
        primary_email: Email,
        password_hash: String,
        address: Address,
        phone_numbers: Vec<PhoneNumber>,
        introduced_by: Option<Uuid>,
    ) -> Self {
        let member = Member::new(
            name,
            organization,
            pseudonym,
            primary_email,
            password_hash,
            address,
            phone_numbers,
            introduced_by,
        );

        Self {
            member,
            author_profile: None,
            admin_profile: None,
            moderator_profile: None,
            unread_messages: Vec::new(),
            unread_notifications: Vec::new(),
            recent_downloads: Vec::new(),
            pending_events: Vec::new(),
        }
    }

    pub fn member_id(&self) -> Uuid {
        self.member.member_id
    }

    pub fn register_as_author(&mut self, orcid: String, bio: String, specialization: String) {
        if self.author_profile.is_none() {
            let author = Author::new(
                self.member.member_id,
                orcid,
                bio,
                specialization,
                self.member.pseudonym.clone().unwrap_or_else(|| self.member.name.clone()),
            );
            
            self.author_profile = Some(author);
            self.pending_events.push(MemberEvent::AuthorRegistered {
                member_id: self.member_id(),
                orcid: author.orcid.clone(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn register_as_admin(&mut self, role: RoleEnum) {
        if self.admin_profile.is_none() {
            let admin = Admin::new(self.member.member_id, role);
            
            self.admin_profile = Some(admin);
            self.pending_events.push(MemberEvent::AdminRegistered {
                member_id: self.member_id(),
                role,
                timestamp: Utc::now(),
            });
        }
    }

    pub fn register_as_moderator(&mut self, domain: String, expertise_areas: Vec<String>) {
        if self.moderator_profile.is_none() {
            let moderator = Moderator::new(
                self.member.member_id,
                domain,
                expertise_areas,
            );
            
            self.moderator_profile = Some(moderator);
            self.pending_events.push(MemberEvent::ModeratorRegistered {
                member_id: self.member_id(),
                domain: moderator.domain.clone(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn activate(&mut self) {
        if self.member.status != MemberStatus::Active {
            self.member.activate();
            self.pending_events.push(MemberEvent::MemberActivated {
                member_id: self.member_id(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn suspend(&mut self, reason: String) {
        if self.member.status != MemberStatus::Suspended {
            self.member.suspend();
            self.pending_events.push(MemberEvent::MemberSuspended {
                member_id: self.member_id(),
                reason,
                timestamp: Utc::now(),
            });
        }
    }

    pub fn ban(&mut self, reason: String) {
        if self.member.status != MemberStatus::Banned {
            self.member.ban();
            self.pending_events.push(MemberEvent::MemberBanned {
                member_id: self.member_id(),
                reason,
                timestamp: Utc::now(),
            });
        }
    }

    pub fn update_verification_matrix(&mut self, matrix: VerificationMatrix) {
        self.member.verification_matrix = Some(matrix);
        self.pending_events.push(MemberEvent::VerificationMatrixUpdated {
            member_id: self.member_id(),
            timestamp: Utc::now(),
        });
    }

    pub fn add_download(&mut self, download: Download) {
        if self.recent_downloads.len() >= 10 {
            self.recent_downloads.remove(0);
        }
        self.recent_downloads.push(download);
    }

    pub fn add_message(&mut self, message: Message) {
        self.unread_messages.push(message);
    }

    pub fn add_notification(&mut self, notification: Notification) {
        self.unread_notifications.push(notification);
    }

    pub fn mark_message_as_read(&mut self, message_id: Uuid) {
        if let Some(message) = self.unread_messages.iter_mut().find(|m| m.message_id == message_id) {
            message.mark_as_read();
        }
    }

    pub fn mark_notification_as_read(&mut self, notification_id: Uuid) {
        if let Some(notification) = self.unread_notifications.iter_mut().find(|n| n.notif_id == notification_id) {
            notification.mark_as_read();
        }
    }

    pub fn is_active(&self) -> bool {
        self.member.is_active()
    }

    pub fn is_author(&self) -> bool {
        self.author_profile.is_some()
    }

    pub fn is_admin(&self) -> bool {
        self.admin_profile.is_some()
    }

    pub fn is_moderator(&self) -> bool {
        self.moderator_profile.is_some()
    }

    pub fn can_download(&self) -> bool {
        self.member.can_download()
    }

    pub fn download_limit_reached(&self) -> bool {
        self.member.download_limit_reached()
    }

    pub fn take_events(&mut self) -> Vec<MemberEvent> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn apply_event(&mut self, event: MemberEvent) {
        match event {
            MemberEvent::MemberCreated { .. } => {}
            MemberEvent::MemberActivated { .. } => {
                self.member.status = MemberStatus::Active;
            }
            MemberEvent::MemberSuspended { reason, .. } => {
                self.member.status = MemberStatus::Suspended;
                // Store suspension reason in member metadata if needed
            }
            MemberEvent::MemberBanned { reason, .. } => {
                self.member.status = MemberStatus::Banned;
                // Store ban reason in member metadata if needed
            }
            MemberEvent::AuthorRegistered { orcid, .. } => {
                if let Some(author) = &mut self.author_profile {
                    author.orcid = orcid;
                }
            }
            MemberEvent::AdminRegistered { role, .. } => {
                if let Some(admin) = &mut self.admin_profile {
                    admin.role = role;
                }
            }
            MemberEvent::ModeratorRegistered { domain, .. } => {
                if let Some(moderator) = &mut self.moderator_profile {
                    moderator.domain = domain;
                }
            }
            MemberEvent::VerificationMatrixUpdated { .. } => {
                // Verification matrix already updated
            }
        }
    }
}
