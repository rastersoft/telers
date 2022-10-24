mod animation;
mod audio;
mod bot_command;
mod bot_command_scope;
mod bot_command_scope_all_chat_administrators;
mod bot_command_scope_all_group_chats;
mod bot_command_scope_all_private_chats;
mod bot_command_scope_chat;
mod bot_command_scope_chat_administrators;
mod bot_command_scope_chat_member;
mod bot_command_scope_default;
mod callback_game;
mod callback_query;
mod chat;
mod chat_administrator_rights;
mod chat_id_kind;
mod chat_invite_link;
mod chat_join_request;
mod chat_location;
mod chat_member;
mod chat_member_administrator;
mod chat_member_banned;
mod chat_member_left;
mod chat_member_member;
mod chat_member_owner;
mod chat_member_restricted;
mod chat_member_updated;
mod chat_permissions;
mod chat_photo;
mod chosen_inline_result;
mod contact;
mod dice;
mod document;
mod encrypted_credentials;
mod encrypted_passport_element;
mod file;
mod force_reply;
mod game;
mod game_high_score;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod inline_query;
mod inline_query_result;
mod inline_query_result_article;
mod inline_query_result_audio;
mod inline_query_result_cached_audio;
mod inline_query_result_cached_document;
mod inline_query_result_cached_gif;
mod inline_query_result_cached_mpeg4_gif;
mod inline_query_result_cached_photo;
mod inline_query_result_cached_sticker;
mod inline_query_result_cached_video;
mod inline_query_result_cached_voice;
mod inline_query_result_contact;
mod inline_query_result_document;
mod inline_query_result_game;
mod inline_query_result_gif;
mod inline_query_result_location;
mod inline_query_result_mpeg4_gif;
mod inline_query_result_photo;
mod inline_query_result_venue;
mod inline_query_result_video;
mod inline_query_result_voice;
mod input_contact_message_content;
mod input_file;
mod input_invoice_message_content;
mod input_location_message_content;
mod input_media;
mod input_media_animation;
mod input_media_audio;
mod input_media_document;
mod input_media_kind;
mod input_media_photo;
mod input_media_video;
mod input_message_content;
mod input_text_message_content;
mod input_venue_message_content;
mod invoice;
mod keyboard_button;
mod keyboard_button_poll_type;
mod labeled_price;
mod location;
mod login_url;
mod mask_position;
mod menu_button;
mod menu_button_commands;
mod menu_button_default;
mod menu_button_web_app;
mod message;
mod message_auto_delete_timer_changed;
mod message_entity;
mod order_info;
mod passport_data;
mod passport_element_error;
mod passport_element_error_data_field;
mod passport_element_error_file;
mod passport_element_error_files;
mod passport_element_error_front_side;
mod passport_element_error_reverse_side;
mod passport_element_error_selfie;
mod passport_element_error_translation_file;
mod passport_element_error_translation_files;
mod passport_element_error_unspecified;
mod passport_file;
mod photo_size;
mod poll;
mod poll_answer;
mod poll_option;
mod pre_checkout_query;
mod proximity_alert_triggered;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod response_parameters;
mod sent_web_app_message;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod sticker_set;
mod update_kind;
mod successful_payment;
mod update;
mod user;
mod user_profile_photos;
mod venue;
mod video;
mod video_chat_ended;
mod video_chat_participants_invited;
mod video_chat_scheduled;
mod video_chat_started;
mod video_note;
mod voice;
mod web_app_data;
mod web_app_info;
mod webhook_info;

pub use animation::Animation;
pub use update_kind::UpdateKind;
pub use audio::Audio;
pub use bot_command::BotCommand;
pub use bot_command_scope::BotCommandScope;
pub use bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;
pub use bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;
pub use bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;
pub use bot_command_scope_chat::BotCommandScopeChat;
pub use bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;
pub use bot_command_scope_chat_member::BotCommandScopeChatMember;
pub use bot_command_scope_default::BotCommandScopeDefault;
pub use callback_game::CallbackGame;
pub use callback_query::CallbackQuery;
pub use chat::Chat;
pub use chat_administrator_rights::ChatAdministratorRights;
pub use chat_id_kind::ChatIdKind;
pub use chat_invite_link::ChatInviteLink;
pub use chat_join_request::ChatJoinRequest;
pub use chat_location::ChatLocation;
pub use chat_member::ChatMember;
pub use chat_member_administrator::ChatMemberAdministrator;
pub use chat_member_banned::ChatMemberBanned;
pub use chat_member_left::ChatMemberLeft;
pub use chat_member_member::ChatMemberMember;
pub use chat_member_owner::ChatMemberOwner;
pub use chat_member_restricted::ChatMemberRestricted;
pub use chat_member_updated::ChatMemberUpdated;
pub use chat_permissions::ChatPermissions;
pub use chat_photo::ChatPhoto;
pub use chosen_inline_result::ChosenInlineResult;
pub use contact::Contact;
pub use dice::{Dice, DiceEmoji};
pub use document::Document;
pub use encrypted_credentials::EncryptedCredentials;
pub use encrypted_passport_element::EncryptedPassportElement;
pub use file::File;
pub use force_reply::ForceReply;
pub use game::Game;
pub use game_high_score::GameHighScore;
pub use inline_keyboard_button::InlineKeyboardButton;
pub use inline_keyboard_markup::InlineKeyboardMarkup;
pub use inline_query::InlineQuery;
pub use inline_query_result::InlineQueryResult;
pub use inline_query_result_article::InlineQueryResultArticle;
pub use inline_query_result_audio::InlineQueryResultAudio;
pub use inline_query_result_cached_audio::InlineQueryResultCachedAudio;
pub use inline_query_result_cached_document::InlineQueryResultCachedDocument;
pub use inline_query_result_cached_gif::InlineQueryResultCachedGif;
pub use inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
pub use inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
pub use inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
pub use inline_query_result_cached_video::InlineQueryResultCachedVideo;
pub use inline_query_result_cached_voice::InlineQueryResultCachedVoice;
pub use inline_query_result_contact::InlineQueryResultContact;
pub use inline_query_result_document::InlineQueryResultDocument;
pub use inline_query_result_game::InlineQueryResultGame;
pub use inline_query_result_gif::InlineQueryResultGif;
pub use inline_query_result_location::InlineQueryResultLocation;
pub use inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
pub use inline_query_result_photo::InlineQueryResultPhoto;
pub use inline_query_result_venue::InlineQueryResultVenue;
pub use inline_query_result_video::InlineQueryResultVideo;
pub use inline_query_result_voice::InlineQueryResultVoice;
pub use input_contact_message_content::InputContactMessageContent;
pub use input_file::InputFile; // TODO: add `FSInputFile` and `URLInputFile`
pub use input_invoice_message_content::InputInvoiceMessageContent;
pub use input_location_message_content::InputLocationMessageContent;
pub use input_media::InputMedia;
pub use input_media_animation::InputMediaAnimation;
pub use input_media_audio::InputMediaAudio;
pub use input_media_document::InputMediaDocument;
pub use input_media_kind::InputMediaKind;
pub use input_media_photo::InputMediaPhoto;
pub use input_media_video::InputMediaVideo;
pub use input_message_content::InputMessageContent;
pub use input_text_message_content::InputTextMessageContent;
pub use input_venue_message_content::InputVenueMessageContent;
pub use invoice::Invoice;
pub use keyboard_button::KeyboardButton;
pub use keyboard_button_poll_type::KeyboardButtonPollType;
pub use labeled_price::LabeledPrice;
pub use location::Location;
pub use login_url::LoginUrl;
pub use mask_position::MaskPosition;
pub use menu_button::MenuButton;
pub use menu_button_commands::MenuButtonCommands;
pub use menu_button_default::MenuButtonDefault;
pub use menu_button_web_app::MenuButtonWebApp;
pub use message::Message;
pub use message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
pub use message_entity::MessageEntity;
pub use order_info::OrderInfo;
pub use passport_data::PassportData;
pub use passport_element_error::PassportElementError;
pub use passport_element_error_data_field::PassportElementErrorDataField;
pub use passport_element_error_file::PassportElementErrorFile;
pub use passport_element_error_files::PassportElementErrorFiles;
pub use passport_element_error_front_side::PassportElementErrorFrontSide;
pub use passport_element_error_reverse_side::PassportElementErrorReverseSide;
pub use passport_element_error_selfie::PassportElementErrorSelfie;
pub use passport_element_error_translation_file::PassportElementErrorTranslationFile;
pub use passport_element_error_translation_files::PassportElementErrorTranslationFiles;
pub use passport_element_error_unspecified::PassportElementErrorUnspecified;
pub use passport_file::PassportFile;
pub use photo_size::PhotoSize;
pub use poll::Poll;
pub use poll_answer::PollAnswer;
pub use poll_option::PollOption;
pub use pre_checkout_query::PreCheckoutQuery;
pub use proximity_alert_triggered::ProximityAlertTriggered;
pub use reply_keyboard_markup::ReplyKeyboardMarkup;
pub use reply_keyboard_remove::ReplyKeyboardRemove;
pub use response_parameters::ResponseParameters;
pub use sent_web_app_message::SentWebAppMessage;
pub use shipping_address::ShippingAddress;
pub use shipping_option::ShippingOption;
pub use shipping_query::ShippingQuery;
pub use sticker::Sticker;
pub use sticker_set::StickerSet;
pub use successful_payment::SuccessfulPayment;
pub use update::Update;
pub use user::User;
pub use user_profile_photos::UserProfilePhotos;
pub use venue::Venue;
pub use video::Video;
pub use video_chat_ended::VideoChatEnded;
pub use video_chat_participants_invited::VideoChatParticipantsInvited;
pub use video_chat_scheduled::VideoChatScheduled;
pub use video_chat_started::VideoChatStarted;
pub use video_note::VideoNote;
pub use voice::Voice;
pub use web_app_data::WebAppData;
pub use web_app_info::WebAppInfo;
pub use webhook_info::WebhookInfo;
