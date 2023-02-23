impl crate::Twitchy {
    /// Checks whether AutoMod would flag the specified message for review.
    pub async fn check_automod_status(&self) {
        todo!()
    }

    /// Allow or deny the message that AutoMod flagged for review.
    pub async fn manage_held_automod_messages(&self) {
        todo!()
    }

    /// Gets the broadcaster’s AutoMod settings.
    pub async fn get_automod_settings(&self) {
        todo!()
    }

    /// Updates the broadcaster’s AutoMod settings.
    pub async fn update_automod_settings(&self) {
        todo!()
    }

    /// Gets all users that the broadcaster banned or put in a timeout.
    pub async fn get_banned_users(&self) {
        todo!()
    }

    /// Bans a user from participating in a broadcaster’s chat room or puts them in a timeout.
    pub async fn ban_user(&self) {
        todo!()
    }

    /// Removes the ban or timeout that was placed on the specified user.
    pub async fn unban_user(&self) {
        todo!()
    }

    /// Gets the broadcaster’s list of non-private, blocked words or phrases.
    pub async fn get_blocked_terms(&self) {
        todo!()
    }

    /// Adds a word or phrase to the broadcaster’s list of blocked terms.
    pub async fn add_blocked_term(&self) {
        todo!()
    }

    /// Removes the word or phrase from the broadcaster’s list of blocked terms.
    pub async fn remove_blocked_term(&self) {
        todo!()
    }

    /// Removes a single chat message or all chat messages from the broadcaster’s chat room.
    pub async fn delete_chat_messages(&self) {
        todo!()
    }

    /// Gets all users allowed to moderate the broadcaster’s chat room.
    pub async fn get_moderators(&self) {
        todo!()
    }

    /// Adds a moderator to the broadcaster’s chat room.
    pub async fn add_channel_moderator(&self) {
        todo!()
    }

    /// Removes a moderator from the broadcaster’s chat room.
    pub async fn remove_channel_moderator(&self) {
        todo!()
    }

    /// Gets a list of the broadcaster’s VIPs.
    pub async fn get_vips(&self) {
        todo!()
    }

    /// Adds the specified user as a VIP in the broadcaster’s channel.
    pub async fn add_channel_vip(&self) {
        todo!()
    }

    /// Removes the specified user as a VIP in the broadcaster’s channel.
    pub async fn remove_channel_vip(&self) {
        todo!()
    }

    /// Activates or deactivates the broadcaster’s Shield Mode.
    pub async fn update_shield_mode_status(&self) {
        todo!()
    }

    /// Gets the broadcaster’s Shield Mode activation status.
    pub async fn get_shield_mode_status(&self) {
        todo!()
    }
}
