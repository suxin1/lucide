//! Yew port of [Lucide](https://lucide.dev/).
//!
//! Lucide is a beautiful & consistent icon toolkit made by the community.
//!
//! See [the Rust Lucide book](https://lucide.rustforweb.org/yew.html) for more documenation.

#![allow(ambiguous_glob_reexports)]

#[cfg(feature = "a-arrow-down")]
mod a_arrow_down;
#[cfg(feature = "a-arrow-up")]
mod a_arrow_up;
#[cfg(feature = "a-large-small")]
mod a_large_small;
#[cfg(feature = "accessibility")]
mod accessibility;
#[cfg(feature = "activity")]
mod activity;
#[cfg(feature = "air-vent")]
mod air_vent;
#[cfg(feature = "airplay")]
mod airplay;
#[cfg(feature = "alarm-clock")]
mod alarm_clock;
#[cfg(feature = "alarm-clock-check")]
mod alarm_clock_check;
#[cfg(feature = "alarm-clock-minus")]
mod alarm_clock_minus;
#[cfg(feature = "alarm-clock-off")]
mod alarm_clock_off;
#[cfg(feature = "alarm-clock-plus")]
mod alarm_clock_plus;
#[cfg(feature = "alarm-smoke")]
mod alarm_smoke;
#[cfg(feature = "album")]
mod album;
#[cfg(feature = "align-center")]
mod align_center;
#[cfg(feature = "align-center-horizontal")]
mod align_center_horizontal;
#[cfg(feature = "align-center-vertical")]
mod align_center_vertical;
#[cfg(feature = "align-end-horizontal")]
mod align_end_horizontal;
#[cfg(feature = "align-end-vertical")]
mod align_end_vertical;
#[cfg(feature = "align-horizontal-distribute-center")]
mod align_horizontal_distribute_center;
#[cfg(feature = "align-horizontal-distribute-end")]
mod align_horizontal_distribute_end;
#[cfg(feature = "align-horizontal-distribute-start")]
mod align_horizontal_distribute_start;
#[cfg(feature = "align-horizontal-justify-center")]
mod align_horizontal_justify_center;
#[cfg(feature = "align-horizontal-justify-end")]
mod align_horizontal_justify_end;
#[cfg(feature = "align-horizontal-justify-start")]
mod align_horizontal_justify_start;
#[cfg(feature = "align-horizontal-space-around")]
mod align_horizontal_space_around;
#[cfg(feature = "align-horizontal-space-between")]
mod align_horizontal_space_between;
#[cfg(feature = "align-justify")]
mod align_justify;
#[cfg(feature = "align-left")]
mod align_left;
#[cfg(feature = "align-right")]
mod align_right;
#[cfg(feature = "align-start-horizontal")]
mod align_start_horizontal;
#[cfg(feature = "align-start-vertical")]
mod align_start_vertical;
#[cfg(feature = "align-vertical-distribute-center")]
mod align_vertical_distribute_center;
#[cfg(feature = "align-vertical-distribute-end")]
mod align_vertical_distribute_end;
#[cfg(feature = "align-vertical-distribute-start")]
mod align_vertical_distribute_start;
#[cfg(feature = "align-vertical-justify-center")]
mod align_vertical_justify_center;
#[cfg(feature = "align-vertical-justify-end")]
mod align_vertical_justify_end;
#[cfg(feature = "align-vertical-justify-start")]
mod align_vertical_justify_start;
#[cfg(feature = "align-vertical-space-around")]
mod align_vertical_space_around;
#[cfg(feature = "align-vertical-space-between")]
mod align_vertical_space_between;
#[cfg(feature = "ambulance")]
mod ambulance;
#[cfg(feature = "ampersand")]
mod ampersand;
#[cfg(feature = "ampersands")]
mod ampersands;
#[cfg(feature = "amphora")]
mod amphora;
#[cfg(feature = "anchor")]
mod anchor;
#[cfg(feature = "angry")]
mod angry;
#[cfg(feature = "annoyed")]
mod annoyed;
#[cfg(feature = "antenna")]
mod antenna;
#[cfg(feature = "anvil")]
mod anvil;
#[cfg(feature = "aperture")]
mod aperture;
#[cfg(feature = "app-window")]
mod app_window;
#[cfg(feature = "app-window-mac")]
mod app_window_mac;
#[cfg(feature = "apple")]
mod apple;
#[cfg(feature = "archive")]
mod archive;
#[cfg(feature = "archive-restore")]
mod archive_restore;
#[cfg(feature = "archive-x")]
mod archive_x;
#[cfg(feature = "armchair")]
mod armchair;
#[cfg(feature = "arrow-big-down")]
mod arrow_big_down;
#[cfg(feature = "arrow-big-down-dash")]
mod arrow_big_down_dash;
#[cfg(feature = "arrow-big-left")]
mod arrow_big_left;
#[cfg(feature = "arrow-big-left-dash")]
mod arrow_big_left_dash;
#[cfg(feature = "arrow-big-right")]
mod arrow_big_right;
#[cfg(feature = "arrow-big-right-dash")]
mod arrow_big_right_dash;
#[cfg(feature = "arrow-big-up")]
mod arrow_big_up;
#[cfg(feature = "arrow-big-up-dash")]
mod arrow_big_up_dash;
#[cfg(feature = "arrow-down")]
mod arrow_down;
#[cfg(feature = "arrow-down-0-1")]
mod arrow_down_0_1;
#[cfg(feature = "arrow-down-1-0")]
mod arrow_down_1_0;
#[cfg(feature = "arrow-down-a-z")]
mod arrow_down_a_z;
#[cfg(feature = "arrow-down-from-line")]
mod arrow_down_from_line;
#[cfg(feature = "arrow-down-left")]
mod arrow_down_left;
#[cfg(feature = "arrow-down-narrow-wide")]
mod arrow_down_narrow_wide;
#[cfg(feature = "arrow-down-right")]
mod arrow_down_right;
#[cfg(feature = "arrow-down-to-dot")]
mod arrow_down_to_dot;
#[cfg(feature = "arrow-down-to-line")]
mod arrow_down_to_line;
#[cfg(feature = "arrow-down-up")]
mod arrow_down_up;
#[cfg(feature = "arrow-down-wide-narrow")]
mod arrow_down_wide_narrow;
#[cfg(feature = "arrow-down-z-a")]
mod arrow_down_z_a;
#[cfg(feature = "arrow-left")]
mod arrow_left;
#[cfg(feature = "arrow-left-from-line")]
mod arrow_left_from_line;
#[cfg(feature = "arrow-left-right")]
mod arrow_left_right;
#[cfg(feature = "arrow-left-to-line")]
mod arrow_left_to_line;
#[cfg(feature = "arrow-right")]
mod arrow_right;
#[cfg(feature = "arrow-right-from-line")]
mod arrow_right_from_line;
#[cfg(feature = "arrow-right-left")]
mod arrow_right_left;
#[cfg(feature = "arrow-right-to-line")]
mod arrow_right_to_line;
#[cfg(feature = "arrow-up")]
mod arrow_up;
#[cfg(feature = "arrow-up-0-1")]
mod arrow_up_0_1;
#[cfg(feature = "arrow-up-1-0")]
mod arrow_up_1_0;
#[cfg(feature = "arrow-up-a-z")]
mod arrow_up_a_z;
#[cfg(feature = "arrow-up-down")]
mod arrow_up_down;
#[cfg(feature = "arrow-up-from-dot")]
mod arrow_up_from_dot;
#[cfg(feature = "arrow-up-from-line")]
mod arrow_up_from_line;
#[cfg(feature = "arrow-up-left")]
mod arrow_up_left;
#[cfg(feature = "arrow-up-narrow-wide")]
mod arrow_up_narrow_wide;
#[cfg(feature = "arrow-up-right")]
mod arrow_up_right;
#[cfg(feature = "arrow-up-to-line")]
mod arrow_up_to_line;
#[cfg(feature = "arrow-up-wide-narrow")]
mod arrow_up_wide_narrow;
#[cfg(feature = "arrow-up-z-a")]
mod arrow_up_z_a;
#[cfg(feature = "arrows-up-from-line")]
mod arrows_up_from_line;
#[cfg(feature = "asterisk")]
mod asterisk;
#[cfg(feature = "at-sign")]
mod at_sign;
#[cfg(feature = "atom")]
mod atom;
#[cfg(feature = "audio-lines")]
mod audio_lines;
#[cfg(feature = "audio-waveform")]
mod audio_waveform;
#[cfg(feature = "award")]
mod award;
#[cfg(feature = "axe")]
mod axe;
#[cfg(feature = "axis-3-d")]
mod axis_3_d;
#[cfg(feature = "baby")]
mod baby;
#[cfg(feature = "backpack")]
mod backpack;
#[cfg(feature = "badge")]
mod badge;
#[cfg(feature = "badge-alert")]
mod badge_alert;
#[cfg(feature = "badge-cent")]
mod badge_cent;
#[cfg(feature = "badge-check")]
mod badge_check;
#[cfg(feature = "badge-dollar-sign")]
mod badge_dollar_sign;
#[cfg(feature = "badge-euro")]
mod badge_euro;
#[cfg(feature = "badge-help")]
mod badge_help;
#[cfg(feature = "badge-indian-rupee")]
mod badge_indian_rupee;
#[cfg(feature = "badge-info")]
mod badge_info;
#[cfg(feature = "badge-japanese-yen")]
mod badge_japanese_yen;
#[cfg(feature = "badge-minus")]
mod badge_minus;
#[cfg(feature = "badge-percent")]
mod badge_percent;
#[cfg(feature = "badge-plus")]
mod badge_plus;
#[cfg(feature = "badge-pound-sterling")]
mod badge_pound_sterling;
#[cfg(feature = "badge-russian-ruble")]
mod badge_russian_ruble;
#[cfg(feature = "badge-swiss-franc")]
mod badge_swiss_franc;
#[cfg(feature = "badge-x")]
mod badge_x;
#[cfg(feature = "baggage-claim")]
mod baggage_claim;
#[cfg(feature = "ban")]
mod ban;
#[cfg(feature = "banana")]
mod banana;
#[cfg(feature = "bandage")]
mod bandage;
#[cfg(feature = "banknote")]
mod banknote;
#[cfg(feature = "barcode")]
mod barcode;
#[cfg(feature = "baseline")]
mod baseline;
#[cfg(feature = "bath")]
mod bath;
#[cfg(feature = "battery")]
mod battery;
#[cfg(feature = "battery-charging")]
mod battery_charging;
#[cfg(feature = "battery-full")]
mod battery_full;
#[cfg(feature = "battery-low")]
mod battery_low;
#[cfg(feature = "battery-medium")]
mod battery_medium;
#[cfg(feature = "battery-warning")]
mod battery_warning;
#[cfg(feature = "beaker")]
mod beaker;
#[cfg(feature = "bean")]
mod bean;
#[cfg(feature = "bean-off")]
mod bean_off;
#[cfg(feature = "bed")]
mod bed;
#[cfg(feature = "bed-double")]
mod bed_double;
#[cfg(feature = "bed-single")]
mod bed_single;
#[cfg(feature = "beef")]
mod beef;
#[cfg(feature = "beer")]
mod beer;
#[cfg(feature = "beer-off")]
mod beer_off;
#[cfg(feature = "bell")]
mod bell;
#[cfg(feature = "bell-dot")]
mod bell_dot;
#[cfg(feature = "bell-electric")]
mod bell_electric;
#[cfg(feature = "bell-minus")]
mod bell_minus;
#[cfg(feature = "bell-off")]
mod bell_off;
#[cfg(feature = "bell-plus")]
mod bell_plus;
#[cfg(feature = "bell-ring")]
mod bell_ring;
#[cfg(feature = "between-horizontal-end")]
mod between_horizontal_end;
#[cfg(feature = "between-horizontal-start")]
mod between_horizontal_start;
#[cfg(feature = "between-vertical-end")]
mod between_vertical_end;
#[cfg(feature = "between-vertical-start")]
mod between_vertical_start;
#[cfg(feature = "biceps-flexed")]
mod biceps_flexed;
#[cfg(feature = "bike")]
mod bike;
#[cfg(feature = "binary")]
mod binary;
#[cfg(feature = "binoculars")]
mod binoculars;
#[cfg(feature = "biohazard")]
mod biohazard;
#[cfg(feature = "bird")]
mod bird;
#[cfg(feature = "bitcoin")]
mod bitcoin;
#[cfg(feature = "blend")]
mod blend;
#[cfg(feature = "blinds")]
mod blinds;
#[cfg(feature = "blocks")]
mod blocks;
#[cfg(feature = "bluetooth")]
mod bluetooth;
#[cfg(feature = "bluetooth-connected")]
mod bluetooth_connected;
#[cfg(feature = "bluetooth-off")]
mod bluetooth_off;
#[cfg(feature = "bluetooth-searching")]
mod bluetooth_searching;
#[cfg(feature = "bold")]
mod bold;
#[cfg(feature = "bolt")]
mod bolt;
#[cfg(feature = "bomb")]
mod bomb;
#[cfg(feature = "bone")]
mod bone;
#[cfg(feature = "book")]
mod book;
#[cfg(feature = "book-a")]
mod book_a;
#[cfg(feature = "book-audio")]
mod book_audio;
#[cfg(feature = "book-check")]
mod book_check;
#[cfg(feature = "book-copy")]
mod book_copy;
#[cfg(feature = "book-dashed")]
mod book_dashed;
#[cfg(feature = "book-down")]
mod book_down;
#[cfg(feature = "book-headphones")]
mod book_headphones;
#[cfg(feature = "book-heart")]
mod book_heart;
#[cfg(feature = "book-image")]
mod book_image;
#[cfg(feature = "book-key")]
mod book_key;
#[cfg(feature = "book-lock")]
mod book_lock;
#[cfg(feature = "book-marked")]
mod book_marked;
#[cfg(feature = "book-minus")]
mod book_minus;
#[cfg(feature = "book-open")]
mod book_open;
#[cfg(feature = "book-open-check")]
mod book_open_check;
#[cfg(feature = "book-open-text")]
mod book_open_text;
#[cfg(feature = "book-plus")]
mod book_plus;
#[cfg(feature = "book-text")]
mod book_text;
#[cfg(feature = "book-type")]
mod book_type;
#[cfg(feature = "book-up")]
mod book_up;
#[cfg(feature = "book-up-2")]
mod book_up_2;
#[cfg(feature = "book-user")]
mod book_user;
#[cfg(feature = "book-x")]
mod book_x;
#[cfg(feature = "bookmark")]
mod bookmark;
#[cfg(feature = "bookmark-check")]
mod bookmark_check;
#[cfg(feature = "bookmark-minus")]
mod bookmark_minus;
#[cfg(feature = "bookmark-plus")]
mod bookmark_plus;
#[cfg(feature = "bookmark-x")]
mod bookmark_x;
#[cfg(feature = "boom-box")]
mod boom_box;
#[cfg(feature = "bot")]
mod bot;
#[cfg(feature = "bot-message-square")]
mod bot_message_square;
#[cfg(feature = "bot-off")]
mod bot_off;
#[cfg(feature = "box")]
mod r#box;
#[cfg(feature = "boxes")]
mod boxes;
#[cfg(feature = "braces")]
mod braces;
#[cfg(feature = "brackets")]
mod brackets;
#[cfg(feature = "brain")]
mod brain;
#[cfg(feature = "brain-circuit")]
mod brain_circuit;
#[cfg(feature = "brain-cog")]
mod brain_cog;
#[cfg(feature = "brick-wall")]
mod brick_wall;
#[cfg(feature = "briefcase")]
mod briefcase;
#[cfg(feature = "briefcase-business")]
mod briefcase_business;
#[cfg(feature = "briefcase-conveyor-belt")]
mod briefcase_conveyor_belt;
#[cfg(feature = "briefcase-medical")]
mod briefcase_medical;
#[cfg(feature = "bring-to-front")]
mod bring_to_front;
#[cfg(feature = "brush")]
mod brush;
#[cfg(feature = "bug")]
mod bug;
#[cfg(feature = "bug-off")]
mod bug_off;
#[cfg(feature = "bug-play")]
mod bug_play;
#[cfg(feature = "building")]
mod building;
#[cfg(feature = "building-2")]
mod building_2;
#[cfg(feature = "bus")]
mod bus;
#[cfg(feature = "bus-front")]
mod bus_front;
#[cfg(feature = "cable")]
mod cable;
#[cfg(feature = "cable-car")]
mod cable_car;
#[cfg(feature = "cake")]
mod cake;
#[cfg(feature = "cake-slice")]
mod cake_slice;
#[cfg(feature = "calculator")]
mod calculator;
#[cfg(feature = "calendar")]
mod calendar;
#[cfg(feature = "calendar-1")]
mod calendar_1;
#[cfg(feature = "calendar-arrow-down")]
mod calendar_arrow_down;
#[cfg(feature = "calendar-arrow-up")]
mod calendar_arrow_up;
#[cfg(feature = "calendar-check")]
mod calendar_check;
#[cfg(feature = "calendar-check-2")]
mod calendar_check_2;
#[cfg(feature = "calendar-clock")]
mod calendar_clock;
#[cfg(feature = "calendar-cog")]
mod calendar_cog;
#[cfg(feature = "calendar-days")]
mod calendar_days;
#[cfg(feature = "calendar-fold")]
mod calendar_fold;
#[cfg(feature = "calendar-heart")]
mod calendar_heart;
#[cfg(feature = "calendar-minus")]
mod calendar_minus;
#[cfg(feature = "calendar-minus-2")]
mod calendar_minus_2;
#[cfg(feature = "calendar-off")]
mod calendar_off;
#[cfg(feature = "calendar-plus")]
mod calendar_plus;
#[cfg(feature = "calendar-plus-2")]
mod calendar_plus_2;
#[cfg(feature = "calendar-range")]
mod calendar_range;
#[cfg(feature = "calendar-search")]
mod calendar_search;
#[cfg(feature = "calendar-x")]
mod calendar_x;
#[cfg(feature = "calendar-x-2")]
mod calendar_x_2;
#[cfg(feature = "camera")]
mod camera;
#[cfg(feature = "camera-off")]
mod camera_off;
#[cfg(feature = "candy")]
mod candy;
#[cfg(feature = "candy-cane")]
mod candy_cane;
#[cfg(feature = "candy-off")]
mod candy_off;
#[cfg(feature = "cannabis")]
mod cannabis;
#[cfg(feature = "captions")]
mod captions;
#[cfg(feature = "captions-off")]
mod captions_off;
#[cfg(feature = "car")]
mod car;
#[cfg(feature = "car-front")]
mod car_front;
#[cfg(feature = "car-taxi-front")]
mod car_taxi_front;
#[cfg(feature = "caravan")]
mod caravan;
#[cfg(feature = "carrot")]
mod carrot;
#[cfg(feature = "case-lower")]
mod case_lower;
#[cfg(feature = "case-sensitive")]
mod case_sensitive;
#[cfg(feature = "case-upper")]
mod case_upper;
#[cfg(feature = "cassette-tape")]
mod cassette_tape;
#[cfg(feature = "cast")]
mod cast;
#[cfg(feature = "castle")]
mod castle;
#[cfg(feature = "cat")]
mod cat;
#[cfg(feature = "cctv")]
mod cctv;
#[cfg(feature = "chart-area")]
mod chart_area;
#[cfg(feature = "chart-bar")]
mod chart_bar;
#[cfg(feature = "chart-bar-big")]
mod chart_bar_big;
#[cfg(feature = "chart-bar-decreasing")]
mod chart_bar_decreasing;
#[cfg(feature = "chart-bar-increasing")]
mod chart_bar_increasing;
#[cfg(feature = "chart-bar-stacked")]
mod chart_bar_stacked;
#[cfg(feature = "chart-candlestick")]
mod chart_candlestick;
#[cfg(feature = "chart-column")]
mod chart_column;
#[cfg(feature = "chart-column-big")]
mod chart_column_big;
#[cfg(feature = "chart-column-decreasing")]
mod chart_column_decreasing;
#[cfg(feature = "chart-column-increasing")]
mod chart_column_increasing;
#[cfg(feature = "chart-column-stacked")]
mod chart_column_stacked;
#[cfg(feature = "chart-gantt")]
mod chart_gantt;
#[cfg(feature = "chart-line")]
mod chart_line;
#[cfg(feature = "chart-network")]
mod chart_network;
#[cfg(feature = "chart-no-axes-column")]
mod chart_no_axes_column;
#[cfg(feature = "chart-no-axes-column-decreasing")]
mod chart_no_axes_column_decreasing;
#[cfg(feature = "chart-no-axes-column-increasing")]
mod chart_no_axes_column_increasing;
#[cfg(feature = "chart-no-axes-combined")]
mod chart_no_axes_combined;
#[cfg(feature = "chart-no-axes-gantt")]
mod chart_no_axes_gantt;
#[cfg(feature = "chart-pie")]
mod chart_pie;
#[cfg(feature = "chart-scatter")]
mod chart_scatter;
#[cfg(feature = "chart-spline")]
mod chart_spline;
#[cfg(feature = "check")]
mod check;
#[cfg(feature = "check-check")]
mod check_check;
#[cfg(feature = "chef-hat")]
mod chef_hat;
#[cfg(feature = "cherry")]
mod cherry;
#[cfg(feature = "chevron-down")]
mod chevron_down;
#[cfg(feature = "chevron-first")]
mod chevron_first;
#[cfg(feature = "chevron-last")]
mod chevron_last;
#[cfg(feature = "chevron-left")]
mod chevron_left;
#[cfg(feature = "chevron-right")]
mod chevron_right;
#[cfg(feature = "chevron-up")]
mod chevron_up;
#[cfg(feature = "chevrons-down")]
mod chevrons_down;
#[cfg(feature = "chevrons-down-up")]
mod chevrons_down_up;
#[cfg(feature = "chevrons-left")]
mod chevrons_left;
#[cfg(feature = "chevrons-left-right")]
mod chevrons_left_right;
#[cfg(feature = "chevrons-left-right-ellipsis")]
mod chevrons_left_right_ellipsis;
#[cfg(feature = "chevrons-right")]
mod chevrons_right;
#[cfg(feature = "chevrons-right-left")]
mod chevrons_right_left;
#[cfg(feature = "chevrons-up")]
mod chevrons_up;
#[cfg(feature = "chevrons-up-down")]
mod chevrons_up_down;
#[cfg(feature = "chrome")]
mod chrome;
#[cfg(feature = "church")]
mod church;
#[cfg(feature = "cigarette")]
mod cigarette;
#[cfg(feature = "cigarette-off")]
mod cigarette_off;
#[cfg(feature = "circle")]
mod circle;
#[cfg(feature = "circle-alert")]
mod circle_alert;
#[cfg(feature = "circle-arrow-down")]
mod circle_arrow_down;
#[cfg(feature = "circle-arrow-left")]
mod circle_arrow_left;
#[cfg(feature = "circle-arrow-out-down-left")]
mod circle_arrow_out_down_left;
#[cfg(feature = "circle-arrow-out-down-right")]
mod circle_arrow_out_down_right;
#[cfg(feature = "circle-arrow-out-up-left")]
mod circle_arrow_out_up_left;
#[cfg(feature = "circle-arrow-out-up-right")]
mod circle_arrow_out_up_right;
#[cfg(feature = "circle-arrow-right")]
mod circle_arrow_right;
#[cfg(feature = "circle-arrow-up")]
mod circle_arrow_up;
#[cfg(feature = "circle-check")]
mod circle_check;
#[cfg(feature = "circle-check-big")]
mod circle_check_big;
#[cfg(feature = "circle-chevron-down")]
mod circle_chevron_down;
#[cfg(feature = "circle-chevron-left")]
mod circle_chevron_left;
#[cfg(feature = "circle-chevron-right")]
mod circle_chevron_right;
#[cfg(feature = "circle-chevron-up")]
mod circle_chevron_up;
#[cfg(feature = "circle-dashed")]
mod circle_dashed;
#[cfg(feature = "circle-divide")]
mod circle_divide;
#[cfg(feature = "circle-dollar-sign")]
mod circle_dollar_sign;
#[cfg(feature = "circle-dot")]
mod circle_dot;
#[cfg(feature = "circle-dot-dashed")]
mod circle_dot_dashed;
#[cfg(feature = "circle-ellipsis")]
mod circle_ellipsis;
#[cfg(feature = "circle-equal")]
mod circle_equal;
#[cfg(feature = "circle-fading-arrow-up")]
mod circle_fading_arrow_up;
#[cfg(feature = "circle-fading-plus")]
mod circle_fading_plus;
#[cfg(feature = "circle-gauge")]
mod circle_gauge;
#[cfg(feature = "circle-help")]
mod circle_help;
#[cfg(feature = "circle-minus")]
mod circle_minus;
#[cfg(feature = "circle-off")]
mod circle_off;
#[cfg(feature = "circle-parking")]
mod circle_parking;
#[cfg(feature = "circle-parking-off")]
mod circle_parking_off;
#[cfg(feature = "circle-pause")]
mod circle_pause;
#[cfg(feature = "circle-percent")]
mod circle_percent;
#[cfg(feature = "circle-play")]
mod circle_play;
#[cfg(feature = "circle-plus")]
mod circle_plus;
#[cfg(feature = "circle-power")]
mod circle_power;
#[cfg(feature = "circle-slash")]
mod circle_slash;
#[cfg(feature = "circle-slash-2")]
mod circle_slash_2;
#[cfg(feature = "circle-stop")]
mod circle_stop;
#[cfg(feature = "circle-user")]
mod circle_user;
#[cfg(feature = "circle-user-round")]
mod circle_user_round;
#[cfg(feature = "circle-x")]
mod circle_x;
#[cfg(feature = "circuit-board")]
mod circuit_board;
#[cfg(feature = "citrus")]
mod citrus;
#[cfg(feature = "clapperboard")]
mod clapperboard;
#[cfg(feature = "clipboard")]
mod clipboard;
#[cfg(feature = "clipboard-check")]
mod clipboard_check;
#[cfg(feature = "clipboard-copy")]
mod clipboard_copy;
#[cfg(feature = "clipboard-list")]
mod clipboard_list;
#[cfg(feature = "clipboard-minus")]
mod clipboard_minus;
#[cfg(feature = "clipboard-paste")]
mod clipboard_paste;
#[cfg(feature = "clipboard-pen")]
mod clipboard_pen;
#[cfg(feature = "clipboard-pen-line")]
mod clipboard_pen_line;
#[cfg(feature = "clipboard-plus")]
mod clipboard_plus;
#[cfg(feature = "clipboard-type")]
mod clipboard_type;
#[cfg(feature = "clipboard-x")]
mod clipboard_x;
#[cfg(feature = "clock")]
mod clock;
#[cfg(feature = "clock-1")]
mod clock_1;
#[cfg(feature = "clock-10")]
mod clock_10;
#[cfg(feature = "clock-11")]
mod clock_11;
#[cfg(feature = "clock-12")]
mod clock_12;
#[cfg(feature = "clock-2")]
mod clock_2;
#[cfg(feature = "clock-3")]
mod clock_3;
#[cfg(feature = "clock-4")]
mod clock_4;
#[cfg(feature = "clock-5")]
mod clock_5;
#[cfg(feature = "clock-6")]
mod clock_6;
#[cfg(feature = "clock-7")]
mod clock_7;
#[cfg(feature = "clock-8")]
mod clock_8;
#[cfg(feature = "clock-9")]
mod clock_9;
#[cfg(feature = "clock-alert")]
mod clock_alert;
#[cfg(feature = "clock-arrow-down")]
mod clock_arrow_down;
#[cfg(feature = "clock-arrow-up")]
mod clock_arrow_up;
#[cfg(feature = "cloud")]
mod cloud;
#[cfg(feature = "cloud-alert")]
mod cloud_alert;
#[cfg(feature = "cloud-cog")]
mod cloud_cog;
#[cfg(feature = "cloud-download")]
mod cloud_download;
#[cfg(feature = "cloud-drizzle")]
mod cloud_drizzle;
#[cfg(feature = "cloud-fog")]
mod cloud_fog;
#[cfg(feature = "cloud-hail")]
mod cloud_hail;
#[cfg(feature = "cloud-lightning")]
mod cloud_lightning;
#[cfg(feature = "cloud-moon")]
mod cloud_moon;
#[cfg(feature = "cloud-moon-rain")]
mod cloud_moon_rain;
#[cfg(feature = "cloud-off")]
mod cloud_off;
#[cfg(feature = "cloud-rain")]
mod cloud_rain;
#[cfg(feature = "cloud-rain-wind")]
mod cloud_rain_wind;
#[cfg(feature = "cloud-snow")]
mod cloud_snow;
#[cfg(feature = "cloud-sun")]
mod cloud_sun;
#[cfg(feature = "cloud-sun-rain")]
mod cloud_sun_rain;
#[cfg(feature = "cloud-upload")]
mod cloud_upload;
#[cfg(feature = "cloudy")]
mod cloudy;
#[cfg(feature = "clover")]
mod clover;
#[cfg(feature = "club")]
mod club;
#[cfg(feature = "code")]
mod code;
#[cfg(feature = "code-xml")]
mod code_xml;
#[cfg(feature = "codepen")]
mod codepen;
#[cfg(feature = "codesandbox")]
mod codesandbox;
#[cfg(feature = "coffee")]
mod coffee;
#[cfg(feature = "cog")]
mod cog;
#[cfg(feature = "coins")]
mod coins;
#[cfg(feature = "columns-2")]
mod columns_2;
#[cfg(feature = "columns-3")]
mod columns_3;
#[cfg(feature = "columns-4")]
mod columns_4;
#[cfg(feature = "combine")]
mod combine;
#[cfg(feature = "command")]
mod command;
#[cfg(feature = "compass")]
mod compass;
#[cfg(feature = "component")]
mod component;
#[cfg(feature = "computer")]
mod computer;
#[cfg(feature = "concierge-bell")]
mod concierge_bell;
#[cfg(feature = "cone")]
mod cone;
#[cfg(feature = "construction")]
mod construction;
#[cfg(feature = "contact")]
mod contact;
#[cfg(feature = "contact-round")]
mod contact_round;
#[cfg(feature = "container")]
mod container;
#[cfg(feature = "contrast")]
mod contrast;
#[cfg(feature = "cookie")]
mod cookie;
#[cfg(feature = "cooking-pot")]
mod cooking_pot;
#[cfg(feature = "copy")]
mod copy;
#[cfg(feature = "copy-check")]
mod copy_check;
#[cfg(feature = "copy-minus")]
mod copy_minus;
#[cfg(feature = "copy-plus")]
mod copy_plus;
#[cfg(feature = "copy-slash")]
mod copy_slash;
#[cfg(feature = "copy-x")]
mod copy_x;
#[cfg(feature = "copyleft")]
mod copyleft;
#[cfg(feature = "copyright")]
mod copyright;
#[cfg(feature = "corner-down-left")]
mod corner_down_left;
#[cfg(feature = "corner-down-right")]
mod corner_down_right;
#[cfg(feature = "corner-left-down")]
mod corner_left_down;
#[cfg(feature = "corner-left-up")]
mod corner_left_up;
#[cfg(feature = "corner-right-down")]
mod corner_right_down;
#[cfg(feature = "corner-right-up")]
mod corner_right_up;
#[cfg(feature = "corner-up-left")]
mod corner_up_left;
#[cfg(feature = "corner-up-right")]
mod corner_up_right;
#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "creative-commons")]
mod creative_commons;
#[cfg(feature = "credit-card")]
mod credit_card;
#[cfg(feature = "croissant")]
mod croissant;
#[cfg(feature = "crop")]
mod crop;
#[cfg(feature = "cross")]
mod cross;
#[cfg(feature = "crosshair")]
mod crosshair;
#[cfg(feature = "crown")]
mod crown;
#[cfg(feature = "cuboid")]
mod cuboid;
#[cfg(feature = "cup-soda")]
mod cup_soda;
#[cfg(feature = "currency")]
mod currency;
#[cfg(feature = "cylinder")]
mod cylinder;
#[cfg(feature = "dam")]
mod dam;
#[cfg(feature = "database")]
mod database;
#[cfg(feature = "database-backup")]
mod database_backup;
#[cfg(feature = "database-zap")]
mod database_zap;
#[cfg(feature = "delete")]
mod delete;
#[cfg(feature = "dessert")]
mod dessert;
#[cfg(feature = "diameter")]
mod diameter;
#[cfg(feature = "diamond")]
mod diamond;
#[cfg(feature = "diamond-minus")]
mod diamond_minus;
#[cfg(feature = "diamond-percent")]
mod diamond_percent;
#[cfg(feature = "diamond-plus")]
mod diamond_plus;
#[cfg(feature = "dice-1")]
mod dice_1;
#[cfg(feature = "dice-2")]
mod dice_2;
#[cfg(feature = "dice-3")]
mod dice_3;
#[cfg(feature = "dice-4")]
mod dice_4;
#[cfg(feature = "dice-5")]
mod dice_5;
#[cfg(feature = "dice-6")]
mod dice_6;
#[cfg(feature = "dices")]
mod dices;
#[cfg(feature = "diff")]
mod diff;
#[cfg(feature = "disc")]
mod disc;
#[cfg(feature = "disc-2")]
mod disc_2;
#[cfg(feature = "disc-3")]
mod disc_3;
#[cfg(feature = "disc-album")]
mod disc_album;
#[cfg(feature = "divide")]
mod divide;
#[cfg(feature = "dna")]
mod dna;
#[cfg(feature = "dna-off")]
mod dna_off;
#[cfg(feature = "dock")]
mod dock;
#[cfg(feature = "dog")]
mod dog;
#[cfg(feature = "dollar-sign")]
mod dollar_sign;
#[cfg(feature = "donut")]
mod donut;
#[cfg(feature = "door-closed")]
mod door_closed;
#[cfg(feature = "door-open")]
mod door_open;
#[cfg(feature = "dot")]
mod dot;
#[cfg(feature = "download")]
mod download;
#[cfg(feature = "drafting-compass")]
mod drafting_compass;
#[cfg(feature = "drama")]
mod drama;
#[cfg(feature = "dribbble")]
mod dribbble;
#[cfg(feature = "drill")]
mod drill;
#[cfg(feature = "droplet")]
mod droplet;
#[cfg(feature = "droplets")]
mod droplets;
#[cfg(feature = "drum")]
mod drum;
#[cfg(feature = "drumstick")]
mod drumstick;
#[cfg(feature = "dumbbell")]
mod dumbbell;
#[cfg(feature = "ear")]
mod ear;
#[cfg(feature = "ear-off")]
mod ear_off;
#[cfg(feature = "earth")]
mod earth;
#[cfg(feature = "earth-lock")]
mod earth_lock;
#[cfg(feature = "eclipse")]
mod eclipse;
#[cfg(feature = "egg")]
mod egg;
#[cfg(feature = "egg-fried")]
mod egg_fried;
#[cfg(feature = "egg-off")]
mod egg_off;
#[cfg(feature = "ellipsis")]
mod ellipsis;
#[cfg(feature = "ellipsis-vertical")]
mod ellipsis_vertical;
#[cfg(feature = "equal")]
mod equal;
#[cfg(feature = "equal-approximately")]
mod equal_approximately;
#[cfg(feature = "equal-not")]
mod equal_not;
#[cfg(feature = "eraser")]
mod eraser;
#[cfg(feature = "ethernet-port")]
mod ethernet_port;
#[cfg(feature = "euro")]
mod euro;
#[cfg(feature = "expand")]
mod expand;
#[cfg(feature = "external-link")]
mod external_link;
#[cfg(feature = "eye")]
mod eye;
#[cfg(feature = "eye-closed")]
mod eye_closed;
#[cfg(feature = "eye-off")]
mod eye_off;
#[cfg(feature = "facebook")]
mod facebook;
#[cfg(feature = "factory")]
mod factory;
#[cfg(feature = "fan")]
mod fan;
#[cfg(feature = "fast-forward")]
mod fast_forward;
#[cfg(feature = "feather")]
mod feather;
#[cfg(feature = "fence")]
mod fence;
#[cfg(feature = "ferris-wheel")]
mod ferris_wheel;
#[cfg(feature = "figma")]
mod figma;
#[cfg(feature = "file")]
mod file;
#[cfg(feature = "file-archive")]
mod file_archive;
#[cfg(feature = "file-audio")]
mod file_audio;
#[cfg(feature = "file-audio-2")]
mod file_audio_2;
#[cfg(feature = "file-axis-3-d")]
mod file_axis_3_d;
#[cfg(feature = "file-badge")]
mod file_badge;
#[cfg(feature = "file-badge-2")]
mod file_badge_2;
#[cfg(feature = "file-box")]
mod file_box;
#[cfg(feature = "file-chart-column")]
mod file_chart_column;
#[cfg(feature = "file-chart-column-increasing")]
mod file_chart_column_increasing;
#[cfg(feature = "file-chart-line")]
mod file_chart_line;
#[cfg(feature = "file-chart-pie")]
mod file_chart_pie;
#[cfg(feature = "file-check")]
mod file_check;
#[cfg(feature = "file-check-2")]
mod file_check_2;
#[cfg(feature = "file-clock")]
mod file_clock;
#[cfg(feature = "file-code")]
mod file_code;
#[cfg(feature = "file-code-2")]
mod file_code_2;
#[cfg(feature = "file-cog")]
mod file_cog;
#[cfg(feature = "file-diff")]
mod file_diff;
#[cfg(feature = "file-digit")]
mod file_digit;
#[cfg(feature = "file-down")]
mod file_down;
#[cfg(feature = "file-heart")]
mod file_heart;
#[cfg(feature = "file-image")]
mod file_image;
#[cfg(feature = "file-input")]
mod file_input;
#[cfg(feature = "file-json")]
mod file_json;
#[cfg(feature = "file-json-2")]
mod file_json_2;
#[cfg(feature = "file-key")]
mod file_key;
#[cfg(feature = "file-key-2")]
mod file_key_2;
#[cfg(feature = "file-lock")]
mod file_lock;
#[cfg(feature = "file-lock-2")]
mod file_lock_2;
#[cfg(feature = "file-minus")]
mod file_minus;
#[cfg(feature = "file-minus-2")]
mod file_minus_2;
#[cfg(feature = "file-music")]
mod file_music;
#[cfg(feature = "file-output")]
mod file_output;
#[cfg(feature = "file-pen")]
mod file_pen;
#[cfg(feature = "file-pen-line")]
mod file_pen_line;
#[cfg(feature = "file-plus")]
mod file_plus;
#[cfg(feature = "file-plus-2")]
mod file_plus_2;
#[cfg(feature = "file-question")]
mod file_question;
#[cfg(feature = "file-scan")]
mod file_scan;
#[cfg(feature = "file-search")]
mod file_search;
#[cfg(feature = "file-search-2")]
mod file_search_2;
#[cfg(feature = "file-sliders")]
mod file_sliders;
#[cfg(feature = "file-spreadsheet")]
mod file_spreadsheet;
#[cfg(feature = "file-stack")]
mod file_stack;
#[cfg(feature = "file-symlink")]
mod file_symlink;
#[cfg(feature = "file-terminal")]
mod file_terminal;
#[cfg(feature = "file-text")]
mod file_text;
#[cfg(feature = "file-type")]
mod file_type;
#[cfg(feature = "file-type-2")]
mod file_type_2;
#[cfg(feature = "file-up")]
mod file_up;
#[cfg(feature = "file-user")]
mod file_user;
#[cfg(feature = "file-video")]
mod file_video;
#[cfg(feature = "file-video-2")]
mod file_video_2;
#[cfg(feature = "file-volume")]
mod file_volume;
#[cfg(feature = "file-volume-2")]
mod file_volume_2;
#[cfg(feature = "file-warning")]
mod file_warning;
#[cfg(feature = "file-x")]
mod file_x;
#[cfg(feature = "file-x-2")]
mod file_x_2;
#[cfg(feature = "files")]
mod files;
#[cfg(feature = "film")]
mod film;
#[cfg(feature = "filter")]
mod filter;
#[cfg(feature = "filter-x")]
mod filter_x;
#[cfg(feature = "fingerprint")]
mod fingerprint;
#[cfg(feature = "fire-extinguisher")]
mod fire_extinguisher;
#[cfg(feature = "fish")]
mod fish;
#[cfg(feature = "fish-off")]
mod fish_off;
#[cfg(feature = "fish-symbol")]
mod fish_symbol;
#[cfg(feature = "flag")]
mod flag;
#[cfg(feature = "flag-off")]
mod flag_off;
#[cfg(feature = "flag-triangle-left")]
mod flag_triangle_left;
#[cfg(feature = "flag-triangle-right")]
mod flag_triangle_right;
#[cfg(feature = "flame")]
mod flame;
#[cfg(feature = "flame-kindling")]
mod flame_kindling;
#[cfg(feature = "flashlight")]
mod flashlight;
#[cfg(feature = "flashlight-off")]
mod flashlight_off;
#[cfg(feature = "flask-conical")]
mod flask_conical;
#[cfg(feature = "flask-conical-off")]
mod flask_conical_off;
#[cfg(feature = "flask-round")]
mod flask_round;
#[cfg(feature = "flip-horizontal")]
mod flip_horizontal;
#[cfg(feature = "flip-horizontal-2")]
mod flip_horizontal_2;
#[cfg(feature = "flip-vertical")]
mod flip_vertical;
#[cfg(feature = "flip-vertical-2")]
mod flip_vertical_2;
#[cfg(feature = "flower")]
mod flower;
#[cfg(feature = "flower-2")]
mod flower_2;
#[cfg(feature = "focus")]
mod focus;
#[cfg(feature = "fold-horizontal")]
mod fold_horizontal;
#[cfg(feature = "fold-vertical")]
mod fold_vertical;
#[cfg(feature = "folder")]
mod folder;
#[cfg(feature = "folder-archive")]
mod folder_archive;
#[cfg(feature = "folder-check")]
mod folder_check;
#[cfg(feature = "folder-clock")]
mod folder_clock;
#[cfg(feature = "folder-closed")]
mod folder_closed;
#[cfg(feature = "folder-code")]
mod folder_code;
#[cfg(feature = "folder-cog")]
mod folder_cog;
#[cfg(feature = "folder-dot")]
mod folder_dot;
#[cfg(feature = "folder-down")]
mod folder_down;
#[cfg(feature = "folder-git")]
mod folder_git;
#[cfg(feature = "folder-git-2")]
mod folder_git_2;
#[cfg(feature = "folder-heart")]
mod folder_heart;
#[cfg(feature = "folder-input")]
mod folder_input;
#[cfg(feature = "folder-kanban")]
mod folder_kanban;
#[cfg(feature = "folder-key")]
mod folder_key;
#[cfg(feature = "folder-lock")]
mod folder_lock;
#[cfg(feature = "folder-minus")]
mod folder_minus;
#[cfg(feature = "folder-open")]
mod folder_open;
#[cfg(feature = "folder-open-dot")]
mod folder_open_dot;
#[cfg(feature = "folder-output")]
mod folder_output;
#[cfg(feature = "folder-pen")]
mod folder_pen;
#[cfg(feature = "folder-plus")]
mod folder_plus;
#[cfg(feature = "folder-root")]
mod folder_root;
#[cfg(feature = "folder-search")]
mod folder_search;
#[cfg(feature = "folder-search-2")]
mod folder_search_2;
#[cfg(feature = "folder-symlink")]
mod folder_symlink;
#[cfg(feature = "folder-sync")]
mod folder_sync;
#[cfg(feature = "folder-tree")]
mod folder_tree;
#[cfg(feature = "folder-up")]
mod folder_up;
#[cfg(feature = "folder-x")]
mod folder_x;
#[cfg(feature = "folders")]
mod folders;
#[cfg(feature = "footprints")]
mod footprints;
#[cfg(feature = "forklift")]
mod forklift;
#[cfg(feature = "forward")]
mod forward;
#[cfg(feature = "frame")]
mod frame;
#[cfg(feature = "framer")]
mod framer;
#[cfg(feature = "frown")]
mod frown;
#[cfg(feature = "fuel")]
mod fuel;
#[cfg(feature = "fullscreen")]
mod fullscreen;
#[cfg(feature = "gallery-horizontal")]
mod gallery_horizontal;
#[cfg(feature = "gallery-horizontal-end")]
mod gallery_horizontal_end;
#[cfg(feature = "gallery-thumbnails")]
mod gallery_thumbnails;
#[cfg(feature = "gallery-vertical")]
mod gallery_vertical;
#[cfg(feature = "gallery-vertical-end")]
mod gallery_vertical_end;
#[cfg(feature = "gamepad")]
mod gamepad;
#[cfg(feature = "gamepad-2")]
mod gamepad_2;
#[cfg(feature = "gauge")]
mod gauge;
#[cfg(feature = "gavel")]
mod gavel;
#[cfg(feature = "gem")]
mod gem;
#[cfg(feature = "ghost")]
mod ghost;
#[cfg(feature = "gift")]
mod gift;
#[cfg(feature = "git-branch")]
mod git_branch;
#[cfg(feature = "git-branch-plus")]
mod git_branch_plus;
#[cfg(feature = "git-commit-horizontal")]
mod git_commit_horizontal;
#[cfg(feature = "git-commit-vertical")]
mod git_commit_vertical;
#[cfg(feature = "git-compare")]
mod git_compare;
#[cfg(feature = "git-compare-arrows")]
mod git_compare_arrows;
#[cfg(feature = "git-fork")]
mod git_fork;
#[cfg(feature = "git-graph")]
mod git_graph;
#[cfg(feature = "git-merge")]
mod git_merge;
#[cfg(feature = "git-pull-request")]
mod git_pull_request;
#[cfg(feature = "git-pull-request-arrow")]
mod git_pull_request_arrow;
#[cfg(feature = "git-pull-request-closed")]
mod git_pull_request_closed;
#[cfg(feature = "git-pull-request-create")]
mod git_pull_request_create;
#[cfg(feature = "git-pull-request-create-arrow")]
mod git_pull_request_create_arrow;
#[cfg(feature = "git-pull-request-draft")]
mod git_pull_request_draft;
#[cfg(feature = "github")]
mod github;
#[cfg(feature = "gitlab")]
mod gitlab;
#[cfg(feature = "glass-water")]
mod glass_water;
#[cfg(feature = "glasses")]
mod glasses;
#[cfg(feature = "globe")]
mod globe;
#[cfg(feature = "globe-lock")]
mod globe_lock;
#[cfg(feature = "goal")]
mod goal;
#[cfg(feature = "grab")]
mod grab;
#[cfg(feature = "graduation-cap")]
mod graduation_cap;
#[cfg(feature = "grape")]
mod grape;
#[cfg(feature = "grid-2-x-2")]
mod grid_2_x_2;
#[cfg(feature = "grid-2-x-2-check")]
mod grid_2_x_2_check;
#[cfg(feature = "grid-2-x-2-plus")]
mod grid_2_x_2_plus;
#[cfg(feature = "grid-2-x-2-x")]
mod grid_2_x_2_x;
#[cfg(feature = "grid-3-x-3")]
mod grid_3_x_3;
#[cfg(feature = "grip")]
mod grip;
#[cfg(feature = "grip-horizontal")]
mod grip_horizontal;
#[cfg(feature = "grip-vertical")]
mod grip_vertical;
#[cfg(feature = "group")]
mod group;
#[cfg(feature = "guitar")]
mod guitar;
#[cfg(feature = "ham")]
mod ham;
#[cfg(feature = "hammer")]
mod hammer;
#[cfg(feature = "hand")]
mod hand;
#[cfg(feature = "hand-coins")]
mod hand_coins;
#[cfg(feature = "hand-heart")]
mod hand_heart;
#[cfg(feature = "hand-helping")]
mod hand_helping;
#[cfg(feature = "hand-metal")]
mod hand_metal;
#[cfg(feature = "hand-platter")]
mod hand_platter;
#[cfg(feature = "handshake")]
mod handshake;
#[cfg(feature = "hard-drive")]
mod hard_drive;
#[cfg(feature = "hard-drive-download")]
mod hard_drive_download;
#[cfg(feature = "hard-drive-upload")]
mod hard_drive_upload;
#[cfg(feature = "hard-hat")]
mod hard_hat;
#[cfg(feature = "hash")]
mod hash;
#[cfg(feature = "haze")]
mod haze;
#[cfg(feature = "hdmi-port")]
mod hdmi_port;
#[cfg(feature = "heading")]
mod heading;
#[cfg(feature = "heading-1")]
mod heading_1;
#[cfg(feature = "heading-2")]
mod heading_2;
#[cfg(feature = "heading-3")]
mod heading_3;
#[cfg(feature = "heading-4")]
mod heading_4;
#[cfg(feature = "heading-5")]
mod heading_5;
#[cfg(feature = "heading-6")]
mod heading_6;
#[cfg(feature = "headphone-off")]
mod headphone_off;
#[cfg(feature = "headphones")]
mod headphones;
#[cfg(feature = "headset")]
mod headset;
#[cfg(feature = "heart")]
mod heart;
#[cfg(feature = "heart-crack")]
mod heart_crack;
#[cfg(feature = "heart-handshake")]
mod heart_handshake;
#[cfg(feature = "heart-off")]
mod heart_off;
#[cfg(feature = "heart-pulse")]
mod heart_pulse;
#[cfg(feature = "heater")]
mod heater;
#[cfg(feature = "hexagon")]
mod hexagon;
#[cfg(feature = "highlighter")]
mod highlighter;
#[cfg(feature = "history")]
mod history;
#[cfg(feature = "hop")]
mod hop;
#[cfg(feature = "hop-off")]
mod hop_off;
#[cfg(feature = "hospital")]
mod hospital;
#[cfg(feature = "hotel")]
mod hotel;
#[cfg(feature = "hourglass")]
mod hourglass;
#[cfg(feature = "house")]
mod house;
#[cfg(feature = "house-plug")]
mod house_plug;
#[cfg(feature = "house-plus")]
mod house_plus;
#[cfg(feature = "ice-cream-bowl")]
mod ice_cream_bowl;
#[cfg(feature = "ice-cream-cone")]
mod ice_cream_cone;
#[cfg(feature = "id-card")]
mod id_card;
#[cfg(feature = "image")]
mod image;
#[cfg(feature = "image-down")]
mod image_down;
#[cfg(feature = "image-minus")]
mod image_minus;
#[cfg(feature = "image-off")]
mod image_off;
#[cfg(feature = "image-play")]
mod image_play;
#[cfg(feature = "image-plus")]
mod image_plus;
#[cfg(feature = "image-up")]
mod image_up;
#[cfg(feature = "images")]
mod images;
#[cfg(feature = "import")]
mod import;
#[cfg(feature = "inbox")]
mod inbox;
#[cfg(feature = "indent-decrease")]
mod indent_decrease;
#[cfg(feature = "indent-increase")]
mod indent_increase;
#[cfg(feature = "indian-rupee")]
mod indian_rupee;
#[cfg(feature = "infinity")]
mod infinity;
#[cfg(feature = "info")]
mod info;
#[cfg(feature = "inspection-panel")]
mod inspection_panel;
#[cfg(feature = "instagram")]
mod instagram;
#[cfg(feature = "italic")]
mod italic;
#[cfg(feature = "iteration-ccw")]
mod iteration_ccw;
#[cfg(feature = "iteration-cw")]
mod iteration_cw;
#[cfg(feature = "japanese-yen")]
mod japanese_yen;
#[cfg(feature = "joystick")]
mod joystick;
#[cfg(feature = "kanban")]
mod kanban;
#[cfg(feature = "key")]
mod key;
#[cfg(feature = "key-round")]
mod key_round;
#[cfg(feature = "key-square")]
mod key_square;
#[cfg(feature = "keyboard")]
mod keyboard;
#[cfg(feature = "keyboard-music")]
mod keyboard_music;
#[cfg(feature = "keyboard-off")]
mod keyboard_off;
#[cfg(feature = "lamp")]
mod lamp;
#[cfg(feature = "lamp-ceiling")]
mod lamp_ceiling;
#[cfg(feature = "lamp-desk")]
mod lamp_desk;
#[cfg(feature = "lamp-floor")]
mod lamp_floor;
#[cfg(feature = "lamp-wall-down")]
mod lamp_wall_down;
#[cfg(feature = "lamp-wall-up")]
mod lamp_wall_up;
#[cfg(feature = "land-plot")]
mod land_plot;
#[cfg(feature = "landmark")]
mod landmark;
#[cfg(feature = "languages")]
mod languages;
#[cfg(feature = "laptop")]
mod laptop;
#[cfg(feature = "laptop-minimal")]
mod laptop_minimal;
#[cfg(feature = "laptop-minimal-check")]
mod laptop_minimal_check;
#[cfg(feature = "lasso")]
mod lasso;
#[cfg(feature = "lasso-select")]
mod lasso_select;
#[cfg(feature = "laugh")]
mod laugh;
#[cfg(feature = "layers")]
mod layers;
#[cfg(feature = "layers-2")]
mod layers_2;
#[cfg(feature = "layers-3")]
mod layers_3;
#[cfg(feature = "layout-dashboard")]
mod layout_dashboard;
#[cfg(feature = "layout-grid")]
mod layout_grid;
#[cfg(feature = "layout-list")]
mod layout_list;
#[cfg(feature = "layout-panel-left")]
mod layout_panel_left;
#[cfg(feature = "layout-panel-top")]
mod layout_panel_top;
#[cfg(feature = "layout-template")]
mod layout_template;
#[cfg(feature = "leaf")]
mod leaf;
#[cfg(feature = "leafy-green")]
mod leafy_green;
#[cfg(feature = "lectern")]
mod lectern;
#[cfg(feature = "letter-text")]
mod letter_text;
#[cfg(feature = "library")]
mod library;
#[cfg(feature = "library-big")]
mod library_big;
#[cfg(feature = "life-buoy")]
mod life_buoy;
#[cfg(feature = "ligature")]
mod ligature;
#[cfg(feature = "lightbulb")]
mod lightbulb;
#[cfg(feature = "lightbulb-off")]
mod lightbulb_off;
#[cfg(feature = "link")]
mod link;
#[cfg(feature = "link-2")]
mod link_2;
#[cfg(feature = "link-2-off")]
mod link_2_off;
#[cfg(feature = "linkedin")]
mod linkedin;
#[cfg(feature = "list")]
mod list;
#[cfg(feature = "list-check")]
mod list_check;
#[cfg(feature = "list-checks")]
mod list_checks;
#[cfg(feature = "list-collapse")]
mod list_collapse;
#[cfg(feature = "list-end")]
mod list_end;
#[cfg(feature = "list-filter")]
mod list_filter;
#[cfg(feature = "list-minus")]
mod list_minus;
#[cfg(feature = "list-music")]
mod list_music;
#[cfg(feature = "list-ordered")]
mod list_ordered;
#[cfg(feature = "list-plus")]
mod list_plus;
#[cfg(feature = "list-restart")]
mod list_restart;
#[cfg(feature = "list-start")]
mod list_start;
#[cfg(feature = "list-todo")]
mod list_todo;
#[cfg(feature = "list-tree")]
mod list_tree;
#[cfg(feature = "list-video")]
mod list_video;
#[cfg(feature = "list-x")]
mod list_x;
#[cfg(feature = "loader")]
mod loader;
#[cfg(feature = "loader-circle")]
mod loader_circle;
#[cfg(feature = "loader-pinwheel")]
mod loader_pinwheel;
#[cfg(feature = "locate")]
mod locate;
#[cfg(feature = "locate-fixed")]
mod locate_fixed;
#[cfg(feature = "locate-off")]
mod locate_off;
#[cfg(feature = "lock")]
mod lock;
#[cfg(feature = "lock-keyhole")]
mod lock_keyhole;
#[cfg(feature = "lock-keyhole-open")]
mod lock_keyhole_open;
#[cfg(feature = "lock-open")]
mod lock_open;
#[cfg(feature = "log-in")]
mod log_in;
#[cfg(feature = "log-out")]
mod log_out;
#[cfg(feature = "logs")]
mod logs;
#[cfg(feature = "lollipop")]
mod lollipop;
#[cfg(feature = "luggage")]
mod luggage;
#[cfg(feature = "magnet")]
mod magnet;
#[cfg(feature = "mail")]
mod mail;
#[cfg(feature = "mail-check")]
mod mail_check;
#[cfg(feature = "mail-minus")]
mod mail_minus;
#[cfg(feature = "mail-open")]
mod mail_open;
#[cfg(feature = "mail-plus")]
mod mail_plus;
#[cfg(feature = "mail-question")]
mod mail_question;
#[cfg(feature = "mail-search")]
mod mail_search;
#[cfg(feature = "mail-warning")]
mod mail_warning;
#[cfg(feature = "mail-x")]
mod mail_x;
#[cfg(feature = "mailbox")]
mod mailbox;
#[cfg(feature = "mails")]
mod mails;
#[cfg(feature = "map")]
mod map;
#[cfg(feature = "map-pin")]
mod map_pin;
#[cfg(feature = "map-pin-check")]
mod map_pin_check;
#[cfg(feature = "map-pin-check-inside")]
mod map_pin_check_inside;
#[cfg(feature = "map-pin-house")]
mod map_pin_house;
#[cfg(feature = "map-pin-minus")]
mod map_pin_minus;
#[cfg(feature = "map-pin-minus-inside")]
mod map_pin_minus_inside;
#[cfg(feature = "map-pin-off")]
mod map_pin_off;
#[cfg(feature = "map-pin-plus")]
mod map_pin_plus;
#[cfg(feature = "map-pin-plus-inside")]
mod map_pin_plus_inside;
#[cfg(feature = "map-pin-x")]
mod map_pin_x;
#[cfg(feature = "map-pin-x-inside")]
mod map_pin_x_inside;
#[cfg(feature = "map-pinned")]
mod map_pinned;
#[cfg(feature = "martini")]
mod martini;
#[cfg(feature = "maximize")]
mod maximize;
#[cfg(feature = "maximize-2")]
mod maximize_2;
#[cfg(feature = "medal")]
mod medal;
#[cfg(feature = "megaphone")]
mod megaphone;
#[cfg(feature = "megaphone-off")]
mod megaphone_off;
#[cfg(feature = "meh")]
mod meh;
#[cfg(feature = "memory-stick")]
mod memory_stick;
#[cfg(feature = "menu")]
mod menu;
#[cfg(feature = "merge")]
mod merge;
#[cfg(feature = "message-circle")]
mod message_circle;
#[cfg(feature = "message-circle-code")]
mod message_circle_code;
#[cfg(feature = "message-circle-dashed")]
mod message_circle_dashed;
#[cfg(feature = "message-circle-heart")]
mod message_circle_heart;
#[cfg(feature = "message-circle-more")]
mod message_circle_more;
#[cfg(feature = "message-circle-off")]
mod message_circle_off;
#[cfg(feature = "message-circle-plus")]
mod message_circle_plus;
#[cfg(feature = "message-circle-question")]
mod message_circle_question;
#[cfg(feature = "message-circle-reply")]
mod message_circle_reply;
#[cfg(feature = "message-circle-warning")]
mod message_circle_warning;
#[cfg(feature = "message-circle-x")]
mod message_circle_x;
#[cfg(feature = "message-square")]
mod message_square;
#[cfg(feature = "message-square-code")]
mod message_square_code;
#[cfg(feature = "message-square-dashed")]
mod message_square_dashed;
#[cfg(feature = "message-square-diff")]
mod message_square_diff;
#[cfg(feature = "message-square-dot")]
mod message_square_dot;
#[cfg(feature = "message-square-heart")]
mod message_square_heart;
#[cfg(feature = "message-square-lock")]
mod message_square_lock;
#[cfg(feature = "message-square-more")]
mod message_square_more;
#[cfg(feature = "message-square-off")]
mod message_square_off;
#[cfg(feature = "message-square-plus")]
mod message_square_plus;
#[cfg(feature = "message-square-quote")]
mod message_square_quote;
#[cfg(feature = "message-square-reply")]
mod message_square_reply;
#[cfg(feature = "message-square-share")]
mod message_square_share;
#[cfg(feature = "message-square-text")]
mod message_square_text;
#[cfg(feature = "message-square-warning")]
mod message_square_warning;
#[cfg(feature = "message-square-x")]
mod message_square_x;
#[cfg(feature = "messages-square")]
mod messages_square;
#[cfg(feature = "mic")]
mod mic;
#[cfg(feature = "mic-off")]
mod mic_off;
#[cfg(feature = "mic-vocal")]
mod mic_vocal;
#[cfg(feature = "microchip")]
mod microchip;
#[cfg(feature = "microscope")]
mod microscope;
#[cfg(feature = "microwave")]
mod microwave;
#[cfg(feature = "milestone")]
mod milestone;
#[cfg(feature = "milk")]
mod milk;
#[cfg(feature = "milk-off")]
mod milk_off;
#[cfg(feature = "minimize")]
mod minimize;
#[cfg(feature = "minimize-2")]
mod minimize_2;
#[cfg(feature = "minus")]
mod minus;
#[cfg(feature = "monitor")]
mod monitor;
#[cfg(feature = "monitor-check")]
mod monitor_check;
#[cfg(feature = "monitor-cog")]
mod monitor_cog;
#[cfg(feature = "monitor-dot")]
mod monitor_dot;
#[cfg(feature = "monitor-down")]
mod monitor_down;
#[cfg(feature = "monitor-off")]
mod monitor_off;
#[cfg(feature = "monitor-pause")]
mod monitor_pause;
#[cfg(feature = "monitor-play")]
mod monitor_play;
#[cfg(feature = "monitor-smartphone")]
mod monitor_smartphone;
#[cfg(feature = "monitor-speaker")]
mod monitor_speaker;
#[cfg(feature = "monitor-stop")]
mod monitor_stop;
#[cfg(feature = "monitor-up")]
mod monitor_up;
#[cfg(feature = "monitor-x")]
mod monitor_x;
#[cfg(feature = "moon")]
mod moon;
#[cfg(feature = "moon-star")]
mod moon_star;
#[cfg(feature = "mountain")]
mod mountain;
#[cfg(feature = "mountain-snow")]
mod mountain_snow;
#[cfg(feature = "mouse")]
mod mouse;
#[cfg(feature = "mouse-off")]
mod mouse_off;
#[cfg(feature = "mouse-pointer")]
mod mouse_pointer;
#[cfg(feature = "mouse-pointer-2")]
mod mouse_pointer_2;
#[cfg(feature = "mouse-pointer-ban")]
mod mouse_pointer_ban;
#[cfg(feature = "mouse-pointer-click")]
mod mouse_pointer_click;
#[cfg(feature = "move")]
mod r#move;
#[cfg(feature = "move-3-d")]
mod move_3_d;
#[cfg(feature = "move-diagonal")]
mod move_diagonal;
#[cfg(feature = "move-diagonal-2")]
mod move_diagonal_2;
#[cfg(feature = "move-down")]
mod move_down;
#[cfg(feature = "move-down-left")]
mod move_down_left;
#[cfg(feature = "move-down-right")]
mod move_down_right;
#[cfg(feature = "move-horizontal")]
mod move_horizontal;
#[cfg(feature = "move-left")]
mod move_left;
#[cfg(feature = "move-right")]
mod move_right;
#[cfg(feature = "move-up")]
mod move_up;
#[cfg(feature = "move-up-left")]
mod move_up_left;
#[cfg(feature = "move-up-right")]
mod move_up_right;
#[cfg(feature = "move-vertical")]
mod move_vertical;
#[cfg(feature = "music")]
mod music;
#[cfg(feature = "music-2")]
mod music_2;
#[cfg(feature = "music-3")]
mod music_3;
#[cfg(feature = "music-4")]
mod music_4;
#[cfg(feature = "navigation")]
mod navigation;
#[cfg(feature = "navigation-2")]
mod navigation_2;
#[cfg(feature = "navigation-2-off")]
mod navigation_2_off;
#[cfg(feature = "navigation-off")]
mod navigation_off;
#[cfg(feature = "network")]
mod network;
#[cfg(feature = "newspaper")]
mod newspaper;
#[cfg(feature = "nfc")]
mod nfc;
#[cfg(feature = "notebook")]
mod notebook;
#[cfg(feature = "notebook-pen")]
mod notebook_pen;
#[cfg(feature = "notebook-tabs")]
mod notebook_tabs;
#[cfg(feature = "notebook-text")]
mod notebook_text;
#[cfg(feature = "notepad-text")]
mod notepad_text;
#[cfg(feature = "notepad-text-dashed")]
mod notepad_text_dashed;
#[cfg(feature = "nut")]
mod nut;
#[cfg(feature = "nut-off")]
mod nut_off;
#[cfg(feature = "octagon")]
mod octagon;
#[cfg(feature = "octagon-alert")]
mod octagon_alert;
#[cfg(feature = "octagon-minus")]
mod octagon_minus;
#[cfg(feature = "octagon-pause")]
mod octagon_pause;
#[cfg(feature = "octagon-x")]
mod octagon_x;
#[cfg(feature = "omega")]
mod omega;
#[cfg(feature = "option")]
mod option;
#[cfg(feature = "orbit")]
mod orbit;
#[cfg(feature = "origami")]
mod origami;
#[cfg(feature = "package")]
mod package;
#[cfg(feature = "package-2")]
mod package_2;
#[cfg(feature = "package-check")]
mod package_check;
#[cfg(feature = "package-minus")]
mod package_minus;
#[cfg(feature = "package-open")]
mod package_open;
#[cfg(feature = "package-plus")]
mod package_plus;
#[cfg(feature = "package-search")]
mod package_search;
#[cfg(feature = "package-x")]
mod package_x;
#[cfg(feature = "paint-bucket")]
mod paint_bucket;
#[cfg(feature = "paint-roller")]
mod paint_roller;
#[cfg(feature = "paintbrush")]
mod paintbrush;
#[cfg(feature = "paintbrush-vertical")]
mod paintbrush_vertical;
#[cfg(feature = "palette")]
mod palette;
#[cfg(feature = "panel-bottom")]
mod panel_bottom;
#[cfg(feature = "panel-bottom-close")]
mod panel_bottom_close;
#[cfg(feature = "panel-bottom-dashed")]
mod panel_bottom_dashed;
#[cfg(feature = "panel-bottom-open")]
mod panel_bottom_open;
#[cfg(feature = "panel-left")]
mod panel_left;
#[cfg(feature = "panel-left-close")]
mod panel_left_close;
#[cfg(feature = "panel-left-dashed")]
mod panel_left_dashed;
#[cfg(feature = "panel-left-open")]
mod panel_left_open;
#[cfg(feature = "panel-right")]
mod panel_right;
#[cfg(feature = "panel-right-close")]
mod panel_right_close;
#[cfg(feature = "panel-right-dashed")]
mod panel_right_dashed;
#[cfg(feature = "panel-right-open")]
mod panel_right_open;
#[cfg(feature = "panel-top")]
mod panel_top;
#[cfg(feature = "panel-top-close")]
mod panel_top_close;
#[cfg(feature = "panel-top-dashed")]
mod panel_top_dashed;
#[cfg(feature = "panel-top-open")]
mod panel_top_open;
#[cfg(feature = "panels-left-bottom")]
mod panels_left_bottom;
#[cfg(feature = "panels-right-bottom")]
mod panels_right_bottom;
#[cfg(feature = "panels-top-left")]
mod panels_top_left;
#[cfg(feature = "paperclip")]
mod paperclip;
#[cfg(feature = "parentheses")]
mod parentheses;
#[cfg(feature = "parking-meter")]
mod parking_meter;
#[cfg(feature = "party-popper")]
mod party_popper;
#[cfg(feature = "pause")]
mod pause;
#[cfg(feature = "paw-print")]
mod paw_print;
#[cfg(feature = "pc-case")]
mod pc_case;
#[cfg(feature = "pen")]
mod pen;
#[cfg(feature = "pen-line")]
mod pen_line;
#[cfg(feature = "pen-off")]
mod pen_off;
#[cfg(feature = "pen-tool")]
mod pen_tool;
#[cfg(feature = "pencil")]
mod pencil;
#[cfg(feature = "pencil-line")]
mod pencil_line;
#[cfg(feature = "pencil-off")]
mod pencil_off;
#[cfg(feature = "pencil-ruler")]
mod pencil_ruler;
#[cfg(feature = "pentagon")]
mod pentagon;
#[cfg(feature = "percent")]
mod percent;
#[cfg(feature = "person-standing")]
mod person_standing;
#[cfg(feature = "philippine-peso")]
mod philippine_peso;
#[cfg(feature = "phone")]
mod phone;
#[cfg(feature = "phone-call")]
mod phone_call;
#[cfg(feature = "phone-forwarded")]
mod phone_forwarded;
#[cfg(feature = "phone-incoming")]
mod phone_incoming;
#[cfg(feature = "phone-missed")]
mod phone_missed;
#[cfg(feature = "phone-off")]
mod phone_off;
#[cfg(feature = "phone-outgoing")]
mod phone_outgoing;
#[cfg(feature = "pi")]
mod pi;
#[cfg(feature = "piano")]
mod piano;
#[cfg(feature = "pickaxe")]
mod pickaxe;
#[cfg(feature = "picture-in-picture")]
mod picture_in_picture;
#[cfg(feature = "picture-in-picture-2")]
mod picture_in_picture_2;
#[cfg(feature = "piggy-bank")]
mod piggy_bank;
#[cfg(feature = "pilcrow")]
mod pilcrow;
#[cfg(feature = "pilcrow-left")]
mod pilcrow_left;
#[cfg(feature = "pilcrow-right")]
mod pilcrow_right;
#[cfg(feature = "pill")]
mod pill;
#[cfg(feature = "pill-bottle")]
mod pill_bottle;
#[cfg(feature = "pin")]
mod pin;
#[cfg(feature = "pin-off")]
mod pin_off;
#[cfg(feature = "pipette")]
mod pipette;
#[cfg(feature = "pizza")]
mod pizza;
#[cfg(feature = "plane")]
mod plane;
#[cfg(feature = "plane-landing")]
mod plane_landing;
#[cfg(feature = "plane-takeoff")]
mod plane_takeoff;
#[cfg(feature = "play")]
mod play;
#[cfg(feature = "plug")]
mod plug;
#[cfg(feature = "plug-2")]
mod plug_2;
#[cfg(feature = "plug-zap")]
mod plug_zap;
#[cfg(feature = "plus")]
mod plus;
#[cfg(feature = "pocket")]
mod pocket;
#[cfg(feature = "pocket-knife")]
mod pocket_knife;
#[cfg(feature = "podcast")]
mod podcast;
#[cfg(feature = "pointer")]
mod pointer;
#[cfg(feature = "pointer-off")]
mod pointer_off;
#[cfg(feature = "popcorn")]
mod popcorn;
#[cfg(feature = "popsicle")]
mod popsicle;
#[cfg(feature = "pound-sterling")]
mod pound_sterling;
#[cfg(feature = "power")]
mod power;
#[cfg(feature = "power-off")]
mod power_off;
#[cfg(feature = "presentation")]
mod presentation;
#[cfg(feature = "printer")]
mod printer;
#[cfg(feature = "printer-check")]
mod printer_check;
#[cfg(feature = "projector")]
mod projector;
#[cfg(feature = "proportions")]
mod proportions;
#[cfg(feature = "puzzle")]
mod puzzle;
#[cfg(feature = "pyramid")]
mod pyramid;
#[cfg(feature = "qr-code")]
mod qr_code;
#[cfg(feature = "quote")]
mod quote;
#[cfg(feature = "rabbit")]
mod rabbit;
#[cfg(feature = "radar")]
mod radar;
#[cfg(feature = "radiation")]
mod radiation;
#[cfg(feature = "radical")]
mod radical;
#[cfg(feature = "radio")]
mod radio;
#[cfg(feature = "radio-receiver")]
mod radio_receiver;
#[cfg(feature = "radio-tower")]
mod radio_tower;
#[cfg(feature = "radius")]
mod radius;
#[cfg(feature = "rail-symbol")]
mod rail_symbol;
#[cfg(feature = "rainbow")]
mod rainbow;
#[cfg(feature = "rat")]
mod rat;
#[cfg(feature = "ratio")]
mod ratio;
#[cfg(feature = "receipt")]
mod receipt;
#[cfg(feature = "receipt-cent")]
mod receipt_cent;
#[cfg(feature = "receipt-euro")]
mod receipt_euro;
#[cfg(feature = "receipt-indian-rupee")]
mod receipt_indian_rupee;
#[cfg(feature = "receipt-japanese-yen")]
mod receipt_japanese_yen;
#[cfg(feature = "receipt-pound-sterling")]
mod receipt_pound_sterling;
#[cfg(feature = "receipt-russian-ruble")]
mod receipt_russian_ruble;
#[cfg(feature = "receipt-swiss-franc")]
mod receipt_swiss_franc;
#[cfg(feature = "receipt-text")]
mod receipt_text;
#[cfg(feature = "rectangle-ellipsis")]
mod rectangle_ellipsis;
#[cfg(feature = "rectangle-horizontal")]
mod rectangle_horizontal;
#[cfg(feature = "rectangle-vertical")]
mod rectangle_vertical;
#[cfg(feature = "recycle")]
mod recycle;
#[cfg(feature = "redo")]
mod redo;
#[cfg(feature = "redo-2")]
mod redo_2;
#[cfg(feature = "redo-dot")]
mod redo_dot;
#[cfg(feature = "refresh-ccw")]
mod refresh_ccw;
#[cfg(feature = "refresh-ccw-dot")]
mod refresh_ccw_dot;
#[cfg(feature = "refresh-cw")]
mod refresh_cw;
#[cfg(feature = "refresh-cw-off")]
mod refresh_cw_off;
#[cfg(feature = "refrigerator")]
mod refrigerator;
#[cfg(feature = "regex")]
mod regex;
#[cfg(feature = "remove-formatting")]
mod remove_formatting;
#[cfg(feature = "repeat")]
mod repeat;
#[cfg(feature = "repeat-1")]
mod repeat_1;
#[cfg(feature = "repeat-2")]
mod repeat_2;
#[cfg(feature = "replace")]
mod replace;
#[cfg(feature = "replace-all")]
mod replace_all;
#[cfg(feature = "reply")]
mod reply;
#[cfg(feature = "reply-all")]
mod reply_all;
#[cfg(feature = "rewind")]
mod rewind;
#[cfg(feature = "ribbon")]
mod ribbon;
#[cfg(feature = "rocket")]
mod rocket;
#[cfg(feature = "rocking-chair")]
mod rocking_chair;
#[cfg(feature = "roller-coaster")]
mod roller_coaster;
#[cfg(feature = "rotate-3-d")]
mod rotate_3_d;
#[cfg(feature = "rotate-ccw")]
mod rotate_ccw;
#[cfg(feature = "rotate-ccw-square")]
mod rotate_ccw_square;
#[cfg(feature = "rotate-cw")]
mod rotate_cw;
#[cfg(feature = "rotate-cw-square")]
mod rotate_cw_square;
#[cfg(feature = "route")]
mod route;
#[cfg(feature = "route-off")]
mod route_off;
#[cfg(feature = "router")]
mod router;
#[cfg(feature = "rows-2")]
mod rows_2;
#[cfg(feature = "rows-3")]
mod rows_3;
#[cfg(feature = "rows-4")]
mod rows_4;
#[cfg(feature = "rss")]
mod rss;
#[cfg(feature = "ruler")]
mod ruler;
#[cfg(feature = "russian-ruble")]
mod russian_ruble;
#[cfg(feature = "sailboat")]
mod sailboat;
#[cfg(feature = "salad")]
mod salad;
#[cfg(feature = "sandwich")]
mod sandwich;
#[cfg(feature = "satellite")]
mod satellite;
#[cfg(feature = "satellite-dish")]
mod satellite_dish;
#[cfg(feature = "save")]
mod save;
#[cfg(feature = "save-all")]
mod save_all;
#[cfg(feature = "save-off")]
mod save_off;
#[cfg(feature = "scale")]
mod scale;
#[cfg(feature = "scale-3-d")]
mod scale_3_d;
#[cfg(feature = "scaling")]
mod scaling;
#[cfg(feature = "scan")]
mod scan;
#[cfg(feature = "scan-barcode")]
mod scan_barcode;
#[cfg(feature = "scan-eye")]
mod scan_eye;
#[cfg(feature = "scan-face")]
mod scan_face;
#[cfg(feature = "scan-line")]
mod scan_line;
#[cfg(feature = "scan-qr-code")]
mod scan_qr_code;
#[cfg(feature = "scan-search")]
mod scan_search;
#[cfg(feature = "scan-text")]
mod scan_text;
#[cfg(feature = "school")]
mod school;
#[cfg(feature = "scissors")]
mod scissors;
#[cfg(feature = "scissors-line-dashed")]
mod scissors_line_dashed;
#[cfg(feature = "screen-share")]
mod screen_share;
#[cfg(feature = "screen-share-off")]
mod screen_share_off;
#[cfg(feature = "scroll")]
mod scroll;
#[cfg(feature = "scroll-text")]
mod scroll_text;
#[cfg(feature = "search")]
mod search;
#[cfg(feature = "search-check")]
mod search_check;
#[cfg(feature = "search-code")]
mod search_code;
#[cfg(feature = "search-slash")]
mod search_slash;
#[cfg(feature = "search-x")]
mod search_x;
#[cfg(feature = "section")]
mod section;
#[cfg(feature = "send")]
mod send;
#[cfg(feature = "send-horizontal")]
mod send_horizontal;
#[cfg(feature = "send-to-back")]
mod send_to_back;
#[cfg(feature = "separator-horizontal")]
mod separator_horizontal;
#[cfg(feature = "separator-vertical")]
mod separator_vertical;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server-cog")]
mod server_cog;
#[cfg(feature = "server-crash")]
mod server_crash;
#[cfg(feature = "server-off")]
mod server_off;
#[cfg(feature = "settings")]
mod settings;
#[cfg(feature = "settings-2")]
mod settings_2;
#[cfg(feature = "shapes")]
mod shapes;
#[cfg(feature = "share")]
mod share;
#[cfg(feature = "share-2")]
mod share_2;
#[cfg(feature = "sheet")]
mod sheet;
#[cfg(feature = "shell")]
mod shell;
#[cfg(feature = "shield")]
mod shield;
#[cfg(feature = "shield-alert")]
mod shield_alert;
#[cfg(feature = "shield-ban")]
mod shield_ban;
#[cfg(feature = "shield-check")]
mod shield_check;
#[cfg(feature = "shield-ellipsis")]
mod shield_ellipsis;
#[cfg(feature = "shield-half")]
mod shield_half;
#[cfg(feature = "shield-minus")]
mod shield_minus;
#[cfg(feature = "shield-off")]
mod shield_off;
#[cfg(feature = "shield-plus")]
mod shield_plus;
#[cfg(feature = "shield-question")]
mod shield_question;
#[cfg(feature = "shield-x")]
mod shield_x;
#[cfg(feature = "ship")]
mod ship;
#[cfg(feature = "ship-wheel")]
mod ship_wheel;
#[cfg(feature = "shirt")]
mod shirt;
#[cfg(feature = "shopping-bag")]
mod shopping_bag;
#[cfg(feature = "shopping-basket")]
mod shopping_basket;
#[cfg(feature = "shopping-cart")]
mod shopping_cart;
#[cfg(feature = "shovel")]
mod shovel;
#[cfg(feature = "shower-head")]
mod shower_head;
#[cfg(feature = "shrink")]
mod shrink;
#[cfg(feature = "shrub")]
mod shrub;
#[cfg(feature = "shuffle")]
mod shuffle;
#[cfg(feature = "sigma")]
mod sigma;
#[cfg(feature = "signal")]
mod signal;
#[cfg(feature = "signal-high")]
mod signal_high;
#[cfg(feature = "signal-low")]
mod signal_low;
#[cfg(feature = "signal-medium")]
mod signal_medium;
#[cfg(feature = "signal-zero")]
mod signal_zero;
#[cfg(feature = "signature")]
mod signature;
#[cfg(feature = "signpost")]
mod signpost;
#[cfg(feature = "signpost-big")]
mod signpost_big;
#[cfg(feature = "siren")]
mod siren;
#[cfg(feature = "skip-back")]
mod skip_back;
#[cfg(feature = "skip-forward")]
mod skip_forward;
#[cfg(feature = "skull")]
mod skull;
#[cfg(feature = "slack")]
mod slack;
#[cfg(feature = "slash")]
mod slash;
#[cfg(feature = "slice")]
mod slice;
#[cfg(feature = "sliders-horizontal")]
mod sliders_horizontal;
#[cfg(feature = "sliders-vertical")]
mod sliders_vertical;
#[cfg(feature = "smartphone")]
mod smartphone;
#[cfg(feature = "smartphone-charging")]
mod smartphone_charging;
#[cfg(feature = "smartphone-nfc")]
mod smartphone_nfc;
#[cfg(feature = "smile")]
mod smile;
#[cfg(feature = "smile-plus")]
mod smile_plus;
#[cfg(feature = "snail")]
mod snail;
#[cfg(feature = "snowflake")]
mod snowflake;
#[cfg(feature = "sofa")]
mod sofa;
#[cfg(feature = "soup")]
mod soup;
#[cfg(feature = "space")]
mod space;
#[cfg(feature = "spade")]
mod spade;
#[cfg(feature = "sparkle")]
mod sparkle;
#[cfg(feature = "sparkles")]
mod sparkles;
#[cfg(feature = "speaker")]
mod speaker;
#[cfg(feature = "speech")]
mod speech;
#[cfg(feature = "spell-check")]
mod spell_check;
#[cfg(feature = "spell-check-2")]
mod spell_check_2;
#[cfg(feature = "spline")]
mod spline;
#[cfg(feature = "split")]
mod split;
#[cfg(feature = "spray-can")]
mod spray_can;
#[cfg(feature = "sprout")]
mod sprout;
#[cfg(feature = "square")]
mod square;
#[cfg(feature = "square-activity")]
mod square_activity;
#[cfg(feature = "square-arrow-down")]
mod square_arrow_down;
#[cfg(feature = "square-arrow-down-left")]
mod square_arrow_down_left;
#[cfg(feature = "square-arrow-down-right")]
mod square_arrow_down_right;
#[cfg(feature = "square-arrow-left")]
mod square_arrow_left;
#[cfg(feature = "square-arrow-out-down-left")]
mod square_arrow_out_down_left;
#[cfg(feature = "square-arrow-out-down-right")]
mod square_arrow_out_down_right;
#[cfg(feature = "square-arrow-out-up-left")]
mod square_arrow_out_up_left;
#[cfg(feature = "square-arrow-out-up-right")]
mod square_arrow_out_up_right;
#[cfg(feature = "square-arrow-right")]
mod square_arrow_right;
#[cfg(feature = "square-arrow-up")]
mod square_arrow_up;
#[cfg(feature = "square-arrow-up-left")]
mod square_arrow_up_left;
#[cfg(feature = "square-arrow-up-right")]
mod square_arrow_up_right;
#[cfg(feature = "square-asterisk")]
mod square_asterisk;
#[cfg(feature = "square-bottom-dashed-scissors")]
mod square_bottom_dashed_scissors;
#[cfg(feature = "square-chart-gantt")]
mod square_chart_gantt;
#[cfg(feature = "square-check")]
mod square_check;
#[cfg(feature = "square-check-big")]
mod square_check_big;
#[cfg(feature = "square-chevron-down")]
mod square_chevron_down;
#[cfg(feature = "square-chevron-left")]
mod square_chevron_left;
#[cfg(feature = "square-chevron-right")]
mod square_chevron_right;
#[cfg(feature = "square-chevron-up")]
mod square_chevron_up;
#[cfg(feature = "square-code")]
mod square_code;
#[cfg(feature = "square-dashed")]
mod square_dashed;
#[cfg(feature = "square-dashed-bottom")]
mod square_dashed_bottom;
#[cfg(feature = "square-dashed-bottom-code")]
mod square_dashed_bottom_code;
#[cfg(feature = "square-dashed-kanban")]
mod square_dashed_kanban;
#[cfg(feature = "square-dashed-mouse-pointer")]
mod square_dashed_mouse_pointer;
#[cfg(feature = "square-divide")]
mod square_divide;
#[cfg(feature = "square-dot")]
mod square_dot;
#[cfg(feature = "square-equal")]
mod square_equal;
#[cfg(feature = "square-function")]
mod square_function;
#[cfg(feature = "square-kanban")]
mod square_kanban;
#[cfg(feature = "square-library")]
mod square_library;
#[cfg(feature = "square-m")]
mod square_m;
#[cfg(feature = "square-menu")]
mod square_menu;
#[cfg(feature = "square-minus")]
mod square_minus;
#[cfg(feature = "square-mouse-pointer")]
mod square_mouse_pointer;
#[cfg(feature = "square-parking")]
mod square_parking;
#[cfg(feature = "square-parking-off")]
mod square_parking_off;
#[cfg(feature = "square-pen")]
mod square_pen;
#[cfg(feature = "square-percent")]
mod square_percent;
#[cfg(feature = "square-pi")]
mod square_pi;
#[cfg(feature = "square-pilcrow")]
mod square_pilcrow;
#[cfg(feature = "square-play")]
mod square_play;
#[cfg(feature = "square-plus")]
mod square_plus;
#[cfg(feature = "square-power")]
mod square_power;
#[cfg(feature = "square-radical")]
mod square_radical;
#[cfg(feature = "square-scissors")]
mod square_scissors;
#[cfg(feature = "square-sigma")]
mod square_sigma;
#[cfg(feature = "square-slash")]
mod square_slash;
#[cfg(feature = "square-split-horizontal")]
mod square_split_horizontal;
#[cfg(feature = "square-split-vertical")]
mod square_split_vertical;
#[cfg(feature = "square-square")]
mod square_square;
#[cfg(feature = "square-stack")]
mod square_stack;
#[cfg(feature = "square-terminal")]
mod square_terminal;
#[cfg(feature = "square-user")]
mod square_user;
#[cfg(feature = "square-user-round")]
mod square_user_round;
#[cfg(feature = "square-x")]
mod square_x;
#[cfg(feature = "squircle")]
mod squircle;
#[cfg(feature = "squirrel")]
mod squirrel;
#[cfg(feature = "stamp")]
mod stamp;
#[cfg(feature = "star")]
mod star;
#[cfg(feature = "star-half")]
mod star_half;
#[cfg(feature = "star-off")]
mod star_off;
#[cfg(feature = "step-back")]
mod step_back;
#[cfg(feature = "step-forward")]
mod step_forward;
#[cfg(feature = "stethoscope")]
mod stethoscope;
#[cfg(feature = "sticker")]
mod sticker;
#[cfg(feature = "sticky-note")]
mod sticky_note;
#[cfg(feature = "store")]
mod store;
#[cfg(feature = "stretch-horizontal")]
mod stretch_horizontal;
#[cfg(feature = "stretch-vertical")]
mod stretch_vertical;
#[cfg(feature = "strikethrough")]
mod strikethrough;
#[cfg(feature = "subscript")]
mod subscript;
#[cfg(feature = "sun")]
mod sun;
#[cfg(feature = "sun-dim")]
mod sun_dim;
#[cfg(feature = "sun-medium")]
mod sun_medium;
#[cfg(feature = "sun-moon")]
mod sun_moon;
#[cfg(feature = "sun-snow")]
mod sun_snow;
#[cfg(feature = "sunrise")]
mod sunrise;
#[cfg(feature = "sunset")]
mod sunset;
#[cfg(feature = "superscript")]
mod superscript;
#[cfg(feature = "swatch-book")]
mod swatch_book;
#[cfg(feature = "swiss-franc")]
mod swiss_franc;
#[cfg(feature = "switch-camera")]
mod switch_camera;
#[cfg(feature = "sword")]
mod sword;
#[cfg(feature = "swords")]
mod swords;
#[cfg(feature = "syringe")]
mod syringe;
#[cfg(feature = "table")]
mod table;
#[cfg(feature = "table-2")]
mod table_2;
#[cfg(feature = "table-cells-merge")]
mod table_cells_merge;
#[cfg(feature = "table-cells-split")]
mod table_cells_split;
#[cfg(feature = "table-columns-split")]
mod table_columns_split;
#[cfg(feature = "table-of-contents")]
mod table_of_contents;
#[cfg(feature = "table-properties")]
mod table_properties;
#[cfg(feature = "table-rows-split")]
mod table_rows_split;
#[cfg(feature = "tablet")]
mod tablet;
#[cfg(feature = "tablet-smartphone")]
mod tablet_smartphone;
#[cfg(feature = "tablets")]
mod tablets;
#[cfg(feature = "tag")]
mod tag;
#[cfg(feature = "tags")]
mod tags;
#[cfg(feature = "tally-1")]
mod tally_1;
#[cfg(feature = "tally-2")]
mod tally_2;
#[cfg(feature = "tally-3")]
mod tally_3;
#[cfg(feature = "tally-4")]
mod tally_4;
#[cfg(feature = "tally-5")]
mod tally_5;
#[cfg(feature = "tangent")]
mod tangent;
#[cfg(feature = "target")]
mod target;
#[cfg(feature = "telescope")]
mod telescope;
#[cfg(feature = "tent")]
mod tent;
#[cfg(feature = "tent-tree")]
mod tent_tree;
#[cfg(feature = "terminal")]
mod terminal;
#[cfg(feature = "test-tube")]
mod test_tube;
#[cfg(feature = "test-tube-diagonal")]
mod test_tube_diagonal;
#[cfg(feature = "test-tubes")]
mod test_tubes;
#[cfg(feature = "text")]
mod text;
#[cfg(feature = "text-cursor")]
mod text_cursor;
#[cfg(feature = "text-cursor-input")]
mod text_cursor_input;
#[cfg(feature = "text-quote")]
mod text_quote;
#[cfg(feature = "text-search")]
mod text_search;
#[cfg(feature = "text-select")]
mod text_select;
#[cfg(feature = "theater")]
mod theater;
#[cfg(feature = "thermometer")]
mod thermometer;
#[cfg(feature = "thermometer-snowflake")]
mod thermometer_snowflake;
#[cfg(feature = "thermometer-sun")]
mod thermometer_sun;
#[cfg(feature = "thumbs-down")]
mod thumbs_down;
#[cfg(feature = "thumbs-up")]
mod thumbs_up;
#[cfg(feature = "ticket")]
mod ticket;
#[cfg(feature = "ticket-check")]
mod ticket_check;
#[cfg(feature = "ticket-minus")]
mod ticket_minus;
#[cfg(feature = "ticket-percent")]
mod ticket_percent;
#[cfg(feature = "ticket-plus")]
mod ticket_plus;
#[cfg(feature = "ticket-slash")]
mod ticket_slash;
#[cfg(feature = "ticket-x")]
mod ticket_x;
#[cfg(feature = "tickets")]
mod tickets;
#[cfg(feature = "tickets-plane")]
mod tickets_plane;
#[cfg(feature = "timer")]
mod timer;
#[cfg(feature = "timer-off")]
mod timer_off;
#[cfg(feature = "timer-reset")]
mod timer_reset;
#[cfg(feature = "toggle-left")]
mod toggle_left;
#[cfg(feature = "toggle-right")]
mod toggle_right;
#[cfg(feature = "toilet")]
mod toilet;
#[cfg(feature = "tornado")]
mod tornado;
#[cfg(feature = "torus")]
mod torus;
#[cfg(feature = "touchpad")]
mod touchpad;
#[cfg(feature = "touchpad-off")]
mod touchpad_off;
#[cfg(feature = "tower-control")]
mod tower_control;
#[cfg(feature = "toy-brick")]
mod toy_brick;
#[cfg(feature = "tractor")]
mod tractor;
#[cfg(feature = "traffic-cone")]
mod traffic_cone;
#[cfg(feature = "train-front")]
mod train_front;
#[cfg(feature = "train-front-tunnel")]
mod train_front_tunnel;
#[cfg(feature = "train-track")]
mod train_track;
#[cfg(feature = "tram-front")]
mod tram_front;
#[cfg(feature = "trash")]
mod trash;
#[cfg(feature = "trash-2")]
mod trash_2;
#[cfg(feature = "tree-deciduous")]
mod tree_deciduous;
#[cfg(feature = "tree-palm")]
mod tree_palm;
#[cfg(feature = "tree-pine")]
mod tree_pine;
#[cfg(feature = "trees")]
mod trees;
#[cfg(feature = "trello")]
mod trello;
#[cfg(feature = "trending-down")]
mod trending_down;
#[cfg(feature = "trending-up")]
mod trending_up;
#[cfg(feature = "trending-up-down")]
mod trending_up_down;
#[cfg(feature = "triangle")]
mod triangle;
#[cfg(feature = "triangle-alert")]
mod triangle_alert;
#[cfg(feature = "triangle-right")]
mod triangle_right;
#[cfg(feature = "trophy")]
mod trophy;
#[cfg(feature = "truck")]
mod truck;
#[cfg(feature = "turtle")]
mod turtle;
#[cfg(feature = "tv")]
mod tv;
#[cfg(feature = "tv-minimal")]
mod tv_minimal;
#[cfg(feature = "tv-minimal-play")]
mod tv_minimal_play;
#[cfg(feature = "twitch")]
mod twitch;
#[cfg(feature = "twitter")]
mod twitter;
#[cfg(feature = "type")]
mod r#type;
#[cfg(feature = "type-outline")]
mod type_outline;
#[cfg(feature = "umbrella")]
mod umbrella;
#[cfg(feature = "umbrella-off")]
mod umbrella_off;
#[cfg(feature = "underline")]
mod underline;
#[cfg(feature = "undo")]
mod undo;
#[cfg(feature = "undo-2")]
mod undo_2;
#[cfg(feature = "undo-dot")]
mod undo_dot;
#[cfg(feature = "unfold-horizontal")]
mod unfold_horizontal;
#[cfg(feature = "unfold-vertical")]
mod unfold_vertical;
#[cfg(feature = "ungroup")]
mod ungroup;
#[cfg(feature = "university")]
mod university;
#[cfg(feature = "unlink")]
mod unlink;
#[cfg(feature = "unlink-2")]
mod unlink_2;
#[cfg(feature = "unplug")]
mod unplug;
#[cfg(feature = "upload")]
mod upload;
#[cfg(feature = "usb")]
mod usb;
#[cfg(feature = "user")]
mod user;
#[cfg(feature = "user-check")]
mod user_check;
#[cfg(feature = "user-cog")]
mod user_cog;
#[cfg(feature = "user-minus")]
mod user_minus;
#[cfg(feature = "user-pen")]
mod user_pen;
#[cfg(feature = "user-plus")]
mod user_plus;
#[cfg(feature = "user-round")]
mod user_round;
#[cfg(feature = "user-round-check")]
mod user_round_check;
#[cfg(feature = "user-round-cog")]
mod user_round_cog;
#[cfg(feature = "user-round-minus")]
mod user_round_minus;
#[cfg(feature = "user-round-pen")]
mod user_round_pen;
#[cfg(feature = "user-round-plus")]
mod user_round_plus;
#[cfg(feature = "user-round-search")]
mod user_round_search;
#[cfg(feature = "user-round-x")]
mod user_round_x;
#[cfg(feature = "user-search")]
mod user_search;
#[cfg(feature = "user-x")]
mod user_x;
#[cfg(feature = "users")]
mod users;
#[cfg(feature = "users-round")]
mod users_round;
#[cfg(feature = "utensils")]
mod utensils;
#[cfg(feature = "utensils-crossed")]
mod utensils_crossed;
#[cfg(feature = "utility-pole")]
mod utility_pole;
#[cfg(feature = "variable")]
mod variable;
#[cfg(feature = "vault")]
mod vault;
#[cfg(feature = "vegan")]
mod vegan;
#[cfg(feature = "venetian-mask")]
mod venetian_mask;
#[cfg(feature = "vibrate")]
mod vibrate;
#[cfg(feature = "vibrate-off")]
mod vibrate_off;
#[cfg(feature = "video")]
mod video;
#[cfg(feature = "video-off")]
mod video_off;
#[cfg(feature = "videotape")]
mod videotape;
#[cfg(feature = "view")]
mod view;
#[cfg(feature = "voicemail")]
mod voicemail;
#[cfg(feature = "volleyball")]
mod volleyball;
#[cfg(feature = "volume")]
mod volume;
#[cfg(feature = "volume-1")]
mod volume_1;
#[cfg(feature = "volume-2")]
mod volume_2;
#[cfg(feature = "volume-off")]
mod volume_off;
#[cfg(feature = "volume-x")]
mod volume_x;
#[cfg(feature = "vote")]
mod vote;
#[cfg(feature = "wallet")]
mod wallet;
#[cfg(feature = "wallet-cards")]
mod wallet_cards;
#[cfg(feature = "wallet-minimal")]
mod wallet_minimal;
#[cfg(feature = "wallpaper")]
mod wallpaper;
#[cfg(feature = "wand")]
mod wand;
#[cfg(feature = "wand-sparkles")]
mod wand_sparkles;
#[cfg(feature = "warehouse")]
mod warehouse;
#[cfg(feature = "washing-machine")]
mod washing_machine;
#[cfg(feature = "watch")]
mod watch;
#[cfg(feature = "waves")]
mod waves;
#[cfg(feature = "waypoints")]
mod waypoints;
#[cfg(feature = "webcam")]
mod webcam;
#[cfg(feature = "webhook")]
mod webhook;
#[cfg(feature = "webhook-off")]
mod webhook_off;
#[cfg(feature = "weight")]
mod weight;
#[cfg(feature = "wheat")]
mod wheat;
#[cfg(feature = "wheat-off")]
mod wheat_off;
#[cfg(feature = "whole-word")]
mod whole_word;
#[cfg(feature = "wifi")]
mod wifi;
#[cfg(feature = "wifi-high")]
mod wifi_high;
#[cfg(feature = "wifi-low")]
mod wifi_low;
#[cfg(feature = "wifi-off")]
mod wifi_off;
#[cfg(feature = "wifi-zero")]
mod wifi_zero;
#[cfg(feature = "wind")]
mod wind;
#[cfg(feature = "wind-arrow-down")]
mod wind_arrow_down;
#[cfg(feature = "wine")]
mod wine;
#[cfg(feature = "wine-off")]
mod wine_off;
#[cfg(feature = "workflow")]
mod workflow;
#[cfg(feature = "worm")]
mod worm;
#[cfg(feature = "wrap-text")]
mod wrap_text;
#[cfg(feature = "wrench")]
mod wrench;
#[cfg(feature = "x")]
mod x;
#[cfg(feature = "youtube")]
mod youtube;
#[cfg(feature = "zap")]
mod zap;
#[cfg(feature = "zap-off")]
mod zap_off;
#[cfg(feature = "zoom-in")]
mod zoom_in;
#[cfg(feature = "zoom-out")]
mod zoom_out;

#[cfg(feature = "a-arrow-down")]
pub use a_arrow_down::*;
#[cfg(feature = "a-arrow-up")]
pub use a_arrow_up::*;
#[cfg(feature = "a-large-small")]
pub use a_large_small::*;
#[cfg(feature = "accessibility")]
pub use accessibility::*;
#[cfg(feature = "activity")]
pub use activity::*;
#[cfg(feature = "air-vent")]
pub use air_vent::*;
#[cfg(feature = "airplay")]
pub use airplay::*;
#[cfg(feature = "alarm-clock")]
pub use alarm_clock::*;
#[cfg(feature = "alarm-clock-check")]
pub use alarm_clock_check::*;
#[cfg(feature = "alarm-clock-minus")]
pub use alarm_clock_minus::*;
#[cfg(feature = "alarm-clock-off")]
pub use alarm_clock_off::*;
#[cfg(feature = "alarm-clock-plus")]
pub use alarm_clock_plus::*;
#[cfg(feature = "alarm-smoke")]
pub use alarm_smoke::*;
#[cfg(feature = "album")]
pub use album::*;
#[cfg(feature = "align-center")]
pub use align_center::*;
#[cfg(feature = "align-center-horizontal")]
pub use align_center_horizontal::*;
#[cfg(feature = "align-center-vertical")]
pub use align_center_vertical::*;
#[cfg(feature = "align-end-horizontal")]
pub use align_end_horizontal::*;
#[cfg(feature = "align-end-vertical")]
pub use align_end_vertical::*;
#[cfg(feature = "align-horizontal-distribute-center")]
pub use align_horizontal_distribute_center::*;
#[cfg(feature = "align-horizontal-distribute-end")]
pub use align_horizontal_distribute_end::*;
#[cfg(feature = "align-horizontal-distribute-start")]
pub use align_horizontal_distribute_start::*;
#[cfg(feature = "align-horizontal-justify-center")]
pub use align_horizontal_justify_center::*;
#[cfg(feature = "align-horizontal-justify-end")]
pub use align_horizontal_justify_end::*;
#[cfg(feature = "align-horizontal-justify-start")]
pub use align_horizontal_justify_start::*;
#[cfg(feature = "align-horizontal-space-around")]
pub use align_horizontal_space_around::*;
#[cfg(feature = "align-horizontal-space-between")]
pub use align_horizontal_space_between::*;
#[cfg(feature = "align-justify")]
pub use align_justify::*;
#[cfg(feature = "align-left")]
pub use align_left::*;
#[cfg(feature = "align-right")]
pub use align_right::*;
#[cfg(feature = "align-start-horizontal")]
pub use align_start_horizontal::*;
#[cfg(feature = "align-start-vertical")]
pub use align_start_vertical::*;
#[cfg(feature = "align-vertical-distribute-center")]
pub use align_vertical_distribute_center::*;
#[cfg(feature = "align-vertical-distribute-end")]
pub use align_vertical_distribute_end::*;
#[cfg(feature = "align-vertical-distribute-start")]
pub use align_vertical_distribute_start::*;
#[cfg(feature = "align-vertical-justify-center")]
pub use align_vertical_justify_center::*;
#[cfg(feature = "align-vertical-justify-end")]
pub use align_vertical_justify_end::*;
#[cfg(feature = "align-vertical-justify-start")]
pub use align_vertical_justify_start::*;
#[cfg(feature = "align-vertical-space-around")]
pub use align_vertical_space_around::*;
#[cfg(feature = "align-vertical-space-between")]
pub use align_vertical_space_between::*;
#[cfg(feature = "ambulance")]
pub use ambulance::*;
#[cfg(feature = "ampersand")]
pub use ampersand::*;
#[cfg(feature = "ampersands")]
pub use ampersands::*;
#[cfg(feature = "amphora")]
pub use amphora::*;
#[cfg(feature = "anchor")]
pub use anchor::*;
#[cfg(feature = "angry")]
pub use angry::*;
#[cfg(feature = "annoyed")]
pub use annoyed::*;
#[cfg(feature = "antenna")]
pub use antenna::*;
#[cfg(feature = "anvil")]
pub use anvil::*;
#[cfg(feature = "aperture")]
pub use aperture::*;
#[cfg(feature = "app-window")]
pub use app_window::*;
#[cfg(feature = "app-window-mac")]
pub use app_window_mac::*;
#[cfg(feature = "apple")]
pub use apple::*;
#[cfg(feature = "archive")]
pub use archive::*;
#[cfg(feature = "archive-restore")]
pub use archive_restore::*;
#[cfg(feature = "archive-x")]
pub use archive_x::*;
#[cfg(feature = "armchair")]
pub use armchair::*;
#[cfg(feature = "arrow-big-down")]
pub use arrow_big_down::*;
#[cfg(feature = "arrow-big-down-dash")]
pub use arrow_big_down_dash::*;
#[cfg(feature = "arrow-big-left")]
pub use arrow_big_left::*;
#[cfg(feature = "arrow-big-left-dash")]
pub use arrow_big_left_dash::*;
#[cfg(feature = "arrow-big-right")]
pub use arrow_big_right::*;
#[cfg(feature = "arrow-big-right-dash")]
pub use arrow_big_right_dash::*;
#[cfg(feature = "arrow-big-up")]
pub use arrow_big_up::*;
#[cfg(feature = "arrow-big-up-dash")]
pub use arrow_big_up_dash::*;
#[cfg(feature = "arrow-down")]
pub use arrow_down::*;
#[cfg(feature = "arrow-down-0-1")]
pub use arrow_down_0_1::*;
#[cfg(feature = "arrow-down-1-0")]
pub use arrow_down_1_0::*;
#[cfg(feature = "arrow-down-a-z")]
pub use arrow_down_a_z::*;
#[cfg(feature = "arrow-down-from-line")]
pub use arrow_down_from_line::*;
#[cfg(feature = "arrow-down-left")]
pub use arrow_down_left::*;
#[cfg(feature = "arrow-down-narrow-wide")]
pub use arrow_down_narrow_wide::*;
#[cfg(feature = "arrow-down-right")]
pub use arrow_down_right::*;
#[cfg(feature = "arrow-down-to-dot")]
pub use arrow_down_to_dot::*;
#[cfg(feature = "arrow-down-to-line")]
pub use arrow_down_to_line::*;
#[cfg(feature = "arrow-down-up")]
pub use arrow_down_up::*;
#[cfg(feature = "arrow-down-wide-narrow")]
pub use arrow_down_wide_narrow::*;
#[cfg(feature = "arrow-down-z-a")]
pub use arrow_down_z_a::*;
#[cfg(feature = "arrow-left")]
pub use arrow_left::*;
#[cfg(feature = "arrow-left-from-line")]
pub use arrow_left_from_line::*;
#[cfg(feature = "arrow-left-right")]
pub use arrow_left_right::*;
#[cfg(feature = "arrow-left-to-line")]
pub use arrow_left_to_line::*;
#[cfg(feature = "arrow-right")]
pub use arrow_right::*;
#[cfg(feature = "arrow-right-from-line")]
pub use arrow_right_from_line::*;
#[cfg(feature = "arrow-right-left")]
pub use arrow_right_left::*;
#[cfg(feature = "arrow-right-to-line")]
pub use arrow_right_to_line::*;
#[cfg(feature = "arrow-up")]
pub use arrow_up::*;
#[cfg(feature = "arrow-up-0-1")]
pub use arrow_up_0_1::*;
#[cfg(feature = "arrow-up-1-0")]
pub use arrow_up_1_0::*;
#[cfg(feature = "arrow-up-a-z")]
pub use arrow_up_a_z::*;
#[cfg(feature = "arrow-up-down")]
pub use arrow_up_down::*;
#[cfg(feature = "arrow-up-from-dot")]
pub use arrow_up_from_dot::*;
#[cfg(feature = "arrow-up-from-line")]
pub use arrow_up_from_line::*;
#[cfg(feature = "arrow-up-left")]
pub use arrow_up_left::*;
#[cfg(feature = "arrow-up-narrow-wide")]
pub use arrow_up_narrow_wide::*;
#[cfg(feature = "arrow-up-right")]
pub use arrow_up_right::*;
#[cfg(feature = "arrow-up-to-line")]
pub use arrow_up_to_line::*;
#[cfg(feature = "arrow-up-wide-narrow")]
pub use arrow_up_wide_narrow::*;
#[cfg(feature = "arrow-up-z-a")]
pub use arrow_up_z_a::*;
#[cfg(feature = "arrows-up-from-line")]
pub use arrows_up_from_line::*;
#[cfg(feature = "asterisk")]
pub use asterisk::*;
#[cfg(feature = "at-sign")]
pub use at_sign::*;
#[cfg(feature = "atom")]
pub use atom::*;
#[cfg(feature = "audio-lines")]
pub use audio_lines::*;
#[cfg(feature = "audio-waveform")]
pub use audio_waveform::*;
#[cfg(feature = "award")]
pub use award::*;
#[cfg(feature = "axe")]
pub use axe::*;
#[cfg(feature = "axis-3-d")]
pub use axis_3_d::*;
#[cfg(feature = "baby")]
pub use baby::*;
#[cfg(feature = "backpack")]
pub use backpack::*;
#[cfg(feature = "badge")]
pub use badge::*;
#[cfg(feature = "badge-alert")]
pub use badge_alert::*;
#[cfg(feature = "badge-cent")]
pub use badge_cent::*;
#[cfg(feature = "badge-check")]
pub use badge_check::*;
#[cfg(feature = "badge-dollar-sign")]
pub use badge_dollar_sign::*;
#[cfg(feature = "badge-euro")]
pub use badge_euro::*;
#[cfg(feature = "badge-help")]
pub use badge_help::*;
#[cfg(feature = "badge-indian-rupee")]
pub use badge_indian_rupee::*;
#[cfg(feature = "badge-info")]
pub use badge_info::*;
#[cfg(feature = "badge-japanese-yen")]
pub use badge_japanese_yen::*;
#[cfg(feature = "badge-minus")]
pub use badge_minus::*;
#[cfg(feature = "badge-percent")]
pub use badge_percent::*;
#[cfg(feature = "badge-plus")]
pub use badge_plus::*;
#[cfg(feature = "badge-pound-sterling")]
pub use badge_pound_sterling::*;
#[cfg(feature = "badge-russian-ruble")]
pub use badge_russian_ruble::*;
#[cfg(feature = "badge-swiss-franc")]
pub use badge_swiss_franc::*;
#[cfg(feature = "badge-x")]
pub use badge_x::*;
#[cfg(feature = "baggage-claim")]
pub use baggage_claim::*;
#[cfg(feature = "ban")]
pub use ban::*;
#[cfg(feature = "banana")]
pub use banana::*;
#[cfg(feature = "bandage")]
pub use bandage::*;
#[cfg(feature = "banknote")]
pub use banknote::*;
#[cfg(feature = "barcode")]
pub use barcode::*;
#[cfg(feature = "baseline")]
pub use baseline::*;
#[cfg(feature = "bath")]
pub use bath::*;
#[cfg(feature = "battery")]
pub use battery::*;
#[cfg(feature = "battery-charging")]
pub use battery_charging::*;
#[cfg(feature = "battery-full")]
pub use battery_full::*;
#[cfg(feature = "battery-low")]
pub use battery_low::*;
#[cfg(feature = "battery-medium")]
pub use battery_medium::*;
#[cfg(feature = "battery-warning")]
pub use battery_warning::*;
#[cfg(feature = "beaker")]
pub use beaker::*;
#[cfg(feature = "bean")]
pub use bean::*;
#[cfg(feature = "bean-off")]
pub use bean_off::*;
#[cfg(feature = "bed")]
pub use bed::*;
#[cfg(feature = "bed-double")]
pub use bed_double::*;
#[cfg(feature = "bed-single")]
pub use bed_single::*;
#[cfg(feature = "beef")]
pub use beef::*;
#[cfg(feature = "beer")]
pub use beer::*;
#[cfg(feature = "beer-off")]
pub use beer_off::*;
#[cfg(feature = "bell")]
pub use bell::*;
#[cfg(feature = "bell-dot")]
pub use bell_dot::*;
#[cfg(feature = "bell-electric")]
pub use bell_electric::*;
#[cfg(feature = "bell-minus")]
pub use bell_minus::*;
#[cfg(feature = "bell-off")]
pub use bell_off::*;
#[cfg(feature = "bell-plus")]
pub use bell_plus::*;
#[cfg(feature = "bell-ring")]
pub use bell_ring::*;
#[cfg(feature = "between-horizontal-end")]
pub use between_horizontal_end::*;
#[cfg(feature = "between-horizontal-start")]
pub use between_horizontal_start::*;
#[cfg(feature = "between-vertical-end")]
pub use between_vertical_end::*;
#[cfg(feature = "between-vertical-start")]
pub use between_vertical_start::*;
#[cfg(feature = "biceps-flexed")]
pub use biceps_flexed::*;
#[cfg(feature = "bike")]
pub use bike::*;
#[cfg(feature = "binary")]
pub use binary::*;
#[cfg(feature = "binoculars")]
pub use binoculars::*;
#[cfg(feature = "biohazard")]
pub use biohazard::*;
#[cfg(feature = "bird")]
pub use bird::*;
#[cfg(feature = "bitcoin")]
pub use bitcoin::*;
#[cfg(feature = "blend")]
pub use blend::*;
#[cfg(feature = "blinds")]
pub use blinds::*;
#[cfg(feature = "blocks")]
pub use blocks::*;
#[cfg(feature = "bluetooth")]
pub use bluetooth::*;
#[cfg(feature = "bluetooth-connected")]
pub use bluetooth_connected::*;
#[cfg(feature = "bluetooth-off")]
pub use bluetooth_off::*;
#[cfg(feature = "bluetooth-searching")]
pub use bluetooth_searching::*;
#[cfg(feature = "bold")]
pub use bold::*;
#[cfg(feature = "bolt")]
pub use bolt::*;
#[cfg(feature = "bomb")]
pub use bomb::*;
#[cfg(feature = "bone")]
pub use bone::*;
#[cfg(feature = "book")]
pub use book::*;
#[cfg(feature = "book-a")]
pub use book_a::*;
#[cfg(feature = "book-audio")]
pub use book_audio::*;
#[cfg(feature = "book-check")]
pub use book_check::*;
#[cfg(feature = "book-copy")]
pub use book_copy::*;
#[cfg(feature = "book-dashed")]
pub use book_dashed::*;
#[cfg(feature = "book-down")]
pub use book_down::*;
#[cfg(feature = "book-headphones")]
pub use book_headphones::*;
#[cfg(feature = "book-heart")]
pub use book_heart::*;
#[cfg(feature = "book-image")]
pub use book_image::*;
#[cfg(feature = "book-key")]
pub use book_key::*;
#[cfg(feature = "book-lock")]
pub use book_lock::*;
#[cfg(feature = "book-marked")]
pub use book_marked::*;
#[cfg(feature = "book-minus")]
pub use book_minus::*;
#[cfg(feature = "book-open")]
pub use book_open::*;
#[cfg(feature = "book-open-check")]
pub use book_open_check::*;
#[cfg(feature = "book-open-text")]
pub use book_open_text::*;
#[cfg(feature = "book-plus")]
pub use book_plus::*;
#[cfg(feature = "book-text")]
pub use book_text::*;
#[cfg(feature = "book-type")]
pub use book_type::*;
#[cfg(feature = "book-up")]
pub use book_up::*;
#[cfg(feature = "book-up-2")]
pub use book_up_2::*;
#[cfg(feature = "book-user")]
pub use book_user::*;
#[cfg(feature = "book-x")]
pub use book_x::*;
#[cfg(feature = "bookmark")]
pub use bookmark::*;
#[cfg(feature = "bookmark-check")]
pub use bookmark_check::*;
#[cfg(feature = "bookmark-minus")]
pub use bookmark_minus::*;
#[cfg(feature = "bookmark-plus")]
pub use bookmark_plus::*;
#[cfg(feature = "bookmark-x")]
pub use bookmark_x::*;
#[cfg(feature = "boom-box")]
pub use boom_box::*;
#[cfg(feature = "bot")]
pub use bot::*;
#[cfg(feature = "bot-message-square")]
pub use bot_message_square::*;
#[cfg(feature = "bot-off")]
pub use bot_off::*;
#[cfg(feature = "boxes")]
pub use boxes::*;
#[cfg(feature = "braces")]
pub use braces::*;
#[cfg(feature = "brackets")]
pub use brackets::*;
#[cfg(feature = "brain")]
pub use brain::*;
#[cfg(feature = "brain-circuit")]
pub use brain_circuit::*;
#[cfg(feature = "brain-cog")]
pub use brain_cog::*;
#[cfg(feature = "brick-wall")]
pub use brick_wall::*;
#[cfg(feature = "briefcase")]
pub use briefcase::*;
#[cfg(feature = "briefcase-business")]
pub use briefcase_business::*;
#[cfg(feature = "briefcase-conveyor-belt")]
pub use briefcase_conveyor_belt::*;
#[cfg(feature = "briefcase-medical")]
pub use briefcase_medical::*;
#[cfg(feature = "bring-to-front")]
pub use bring_to_front::*;
#[cfg(feature = "brush")]
pub use brush::*;
#[cfg(feature = "bug")]
pub use bug::*;
#[cfg(feature = "bug-off")]
pub use bug_off::*;
#[cfg(feature = "bug-play")]
pub use bug_play::*;
#[cfg(feature = "building")]
pub use building::*;
#[cfg(feature = "building-2")]
pub use building_2::*;
#[cfg(feature = "bus")]
pub use bus::*;
#[cfg(feature = "bus-front")]
pub use bus_front::*;
#[cfg(feature = "cable")]
pub use cable::*;
#[cfg(feature = "cable-car")]
pub use cable_car::*;
#[cfg(feature = "cake")]
pub use cake::*;
#[cfg(feature = "cake-slice")]
pub use cake_slice::*;
#[cfg(feature = "calculator")]
pub use calculator::*;
#[cfg(feature = "calendar")]
pub use calendar::*;
#[cfg(feature = "calendar-1")]
pub use calendar_1::*;
#[cfg(feature = "calendar-arrow-down")]
pub use calendar_arrow_down::*;
#[cfg(feature = "calendar-arrow-up")]
pub use calendar_arrow_up::*;
#[cfg(feature = "calendar-check")]
pub use calendar_check::*;
#[cfg(feature = "calendar-check-2")]
pub use calendar_check_2::*;
#[cfg(feature = "calendar-clock")]
pub use calendar_clock::*;
#[cfg(feature = "calendar-cog")]
pub use calendar_cog::*;
#[cfg(feature = "calendar-days")]
pub use calendar_days::*;
#[cfg(feature = "calendar-fold")]
pub use calendar_fold::*;
#[cfg(feature = "calendar-heart")]
pub use calendar_heart::*;
#[cfg(feature = "calendar-minus")]
pub use calendar_minus::*;
#[cfg(feature = "calendar-minus-2")]
pub use calendar_minus_2::*;
#[cfg(feature = "calendar-off")]
pub use calendar_off::*;
#[cfg(feature = "calendar-plus")]
pub use calendar_plus::*;
#[cfg(feature = "calendar-plus-2")]
pub use calendar_plus_2::*;
#[cfg(feature = "calendar-range")]
pub use calendar_range::*;
#[cfg(feature = "calendar-search")]
pub use calendar_search::*;
#[cfg(feature = "calendar-x")]
pub use calendar_x::*;
#[cfg(feature = "calendar-x-2")]
pub use calendar_x_2::*;
#[cfg(feature = "camera")]
pub use camera::*;
#[cfg(feature = "camera-off")]
pub use camera_off::*;
#[cfg(feature = "candy")]
pub use candy::*;
#[cfg(feature = "candy-cane")]
pub use candy_cane::*;
#[cfg(feature = "candy-off")]
pub use candy_off::*;
#[cfg(feature = "cannabis")]
pub use cannabis::*;
#[cfg(feature = "captions")]
pub use captions::*;
#[cfg(feature = "captions-off")]
pub use captions_off::*;
#[cfg(feature = "car")]
pub use car::*;
#[cfg(feature = "car-front")]
pub use car_front::*;
#[cfg(feature = "car-taxi-front")]
pub use car_taxi_front::*;
#[cfg(feature = "caravan")]
pub use caravan::*;
#[cfg(feature = "carrot")]
pub use carrot::*;
#[cfg(feature = "case-lower")]
pub use case_lower::*;
#[cfg(feature = "case-sensitive")]
pub use case_sensitive::*;
#[cfg(feature = "case-upper")]
pub use case_upper::*;
#[cfg(feature = "cassette-tape")]
pub use cassette_tape::*;
#[cfg(feature = "cast")]
pub use cast::*;
#[cfg(feature = "castle")]
pub use castle::*;
#[cfg(feature = "cat")]
pub use cat::*;
#[cfg(feature = "cctv")]
pub use cctv::*;
#[cfg(feature = "chart-area")]
pub use chart_area::*;
#[cfg(feature = "chart-bar")]
pub use chart_bar::*;
#[cfg(feature = "chart-bar-big")]
pub use chart_bar_big::*;
#[cfg(feature = "chart-bar-decreasing")]
pub use chart_bar_decreasing::*;
#[cfg(feature = "chart-bar-increasing")]
pub use chart_bar_increasing::*;
#[cfg(feature = "chart-bar-stacked")]
pub use chart_bar_stacked::*;
#[cfg(feature = "chart-candlestick")]
pub use chart_candlestick::*;
#[cfg(feature = "chart-column")]
pub use chart_column::*;
#[cfg(feature = "chart-column-big")]
pub use chart_column_big::*;
#[cfg(feature = "chart-column-decreasing")]
pub use chart_column_decreasing::*;
#[cfg(feature = "chart-column-increasing")]
pub use chart_column_increasing::*;
#[cfg(feature = "chart-column-stacked")]
pub use chart_column_stacked::*;
#[cfg(feature = "chart-gantt")]
pub use chart_gantt::*;
#[cfg(feature = "chart-line")]
pub use chart_line::*;
#[cfg(feature = "chart-network")]
pub use chart_network::*;
#[cfg(feature = "chart-no-axes-column")]
pub use chart_no_axes_column::*;
#[cfg(feature = "chart-no-axes-column-decreasing")]
pub use chart_no_axes_column_decreasing::*;
#[cfg(feature = "chart-no-axes-column-increasing")]
pub use chart_no_axes_column_increasing::*;
#[cfg(feature = "chart-no-axes-combined")]
pub use chart_no_axes_combined::*;
#[cfg(feature = "chart-no-axes-gantt")]
pub use chart_no_axes_gantt::*;
#[cfg(feature = "chart-pie")]
pub use chart_pie::*;
#[cfg(feature = "chart-scatter")]
pub use chart_scatter::*;
#[cfg(feature = "chart-spline")]
pub use chart_spline::*;
#[cfg(feature = "check")]
pub use check::*;
#[cfg(feature = "check-check")]
pub use check_check::*;
#[cfg(feature = "chef-hat")]
pub use chef_hat::*;
#[cfg(feature = "cherry")]
pub use cherry::*;
#[cfg(feature = "chevron-down")]
pub use chevron_down::*;
#[cfg(feature = "chevron-first")]
pub use chevron_first::*;
#[cfg(feature = "chevron-last")]
pub use chevron_last::*;
#[cfg(feature = "chevron-left")]
pub use chevron_left::*;
#[cfg(feature = "chevron-right")]
pub use chevron_right::*;
#[cfg(feature = "chevron-up")]
pub use chevron_up::*;
#[cfg(feature = "chevrons-down")]
pub use chevrons_down::*;
#[cfg(feature = "chevrons-down-up")]
pub use chevrons_down_up::*;
#[cfg(feature = "chevrons-left")]
pub use chevrons_left::*;
#[cfg(feature = "chevrons-left-right")]
pub use chevrons_left_right::*;
#[cfg(feature = "chevrons-left-right-ellipsis")]
pub use chevrons_left_right_ellipsis::*;
#[cfg(feature = "chevrons-right")]
pub use chevrons_right::*;
#[cfg(feature = "chevrons-right-left")]
pub use chevrons_right_left::*;
#[cfg(feature = "chevrons-up")]
pub use chevrons_up::*;
#[cfg(feature = "chevrons-up-down")]
pub use chevrons_up_down::*;
#[cfg(feature = "chrome")]
pub use chrome::*;
#[cfg(feature = "church")]
pub use church::*;
#[cfg(feature = "cigarette")]
pub use cigarette::*;
#[cfg(feature = "cigarette-off")]
pub use cigarette_off::*;
#[cfg(feature = "circle")]
pub use circle::*;
#[cfg(feature = "circle-alert")]
pub use circle_alert::*;
#[cfg(feature = "circle-arrow-down")]
pub use circle_arrow_down::*;
#[cfg(feature = "circle-arrow-left")]
pub use circle_arrow_left::*;
#[cfg(feature = "circle-arrow-out-down-left")]
pub use circle_arrow_out_down_left::*;
#[cfg(feature = "circle-arrow-out-down-right")]
pub use circle_arrow_out_down_right::*;
#[cfg(feature = "circle-arrow-out-up-left")]
pub use circle_arrow_out_up_left::*;
#[cfg(feature = "circle-arrow-out-up-right")]
pub use circle_arrow_out_up_right::*;
#[cfg(feature = "circle-arrow-right")]
pub use circle_arrow_right::*;
#[cfg(feature = "circle-arrow-up")]
pub use circle_arrow_up::*;
#[cfg(feature = "circle-check")]
pub use circle_check::*;
#[cfg(feature = "circle-check-big")]
pub use circle_check_big::*;
#[cfg(feature = "circle-chevron-down")]
pub use circle_chevron_down::*;
#[cfg(feature = "circle-chevron-left")]
pub use circle_chevron_left::*;
#[cfg(feature = "circle-chevron-right")]
pub use circle_chevron_right::*;
#[cfg(feature = "circle-chevron-up")]
pub use circle_chevron_up::*;
#[cfg(feature = "circle-dashed")]
pub use circle_dashed::*;
#[cfg(feature = "circle-divide")]
pub use circle_divide::*;
#[cfg(feature = "circle-dollar-sign")]
pub use circle_dollar_sign::*;
#[cfg(feature = "circle-dot")]
pub use circle_dot::*;
#[cfg(feature = "circle-dot-dashed")]
pub use circle_dot_dashed::*;
#[cfg(feature = "circle-ellipsis")]
pub use circle_ellipsis::*;
#[cfg(feature = "circle-equal")]
pub use circle_equal::*;
#[cfg(feature = "circle-fading-arrow-up")]
pub use circle_fading_arrow_up::*;
#[cfg(feature = "circle-fading-plus")]
pub use circle_fading_plus::*;
#[cfg(feature = "circle-gauge")]
pub use circle_gauge::*;
#[cfg(feature = "circle-help")]
pub use circle_help::*;
#[cfg(feature = "circle-minus")]
pub use circle_minus::*;
#[cfg(feature = "circle-off")]
pub use circle_off::*;
#[cfg(feature = "circle-parking")]
pub use circle_parking::*;
#[cfg(feature = "circle-parking-off")]
pub use circle_parking_off::*;
#[cfg(feature = "circle-pause")]
pub use circle_pause::*;
#[cfg(feature = "circle-percent")]
pub use circle_percent::*;
#[cfg(feature = "circle-play")]
pub use circle_play::*;
#[cfg(feature = "circle-plus")]
pub use circle_plus::*;
#[cfg(feature = "circle-power")]
pub use circle_power::*;
#[cfg(feature = "circle-slash")]
pub use circle_slash::*;
#[cfg(feature = "circle-slash-2")]
pub use circle_slash_2::*;
#[cfg(feature = "circle-stop")]
pub use circle_stop::*;
#[cfg(feature = "circle-user")]
pub use circle_user::*;
#[cfg(feature = "circle-user-round")]
pub use circle_user_round::*;
#[cfg(feature = "circle-x")]
pub use circle_x::*;
#[cfg(feature = "circuit-board")]
pub use circuit_board::*;
#[cfg(feature = "citrus")]
pub use citrus::*;
#[cfg(feature = "clapperboard")]
pub use clapperboard::*;
#[cfg(feature = "clipboard")]
pub use clipboard::*;
#[cfg(feature = "clipboard-check")]
pub use clipboard_check::*;
#[cfg(feature = "clipboard-copy")]
pub use clipboard_copy::*;
#[cfg(feature = "clipboard-list")]
pub use clipboard_list::*;
#[cfg(feature = "clipboard-minus")]
pub use clipboard_minus::*;
#[cfg(feature = "clipboard-paste")]
pub use clipboard_paste::*;
#[cfg(feature = "clipboard-pen")]
pub use clipboard_pen::*;
#[cfg(feature = "clipboard-pen-line")]
pub use clipboard_pen_line::*;
#[cfg(feature = "clipboard-plus")]
pub use clipboard_plus::*;
#[cfg(feature = "clipboard-type")]
pub use clipboard_type::*;
#[cfg(feature = "clipboard-x")]
pub use clipboard_x::*;
#[cfg(feature = "clock")]
pub use clock::*;
#[cfg(feature = "clock-1")]
pub use clock_1::*;
#[cfg(feature = "clock-10")]
pub use clock_10::*;
#[cfg(feature = "clock-11")]
pub use clock_11::*;
#[cfg(feature = "clock-12")]
pub use clock_12::*;
#[cfg(feature = "clock-2")]
pub use clock_2::*;
#[cfg(feature = "clock-3")]
pub use clock_3::*;
#[cfg(feature = "clock-4")]
pub use clock_4::*;
#[cfg(feature = "clock-5")]
pub use clock_5::*;
#[cfg(feature = "clock-6")]
pub use clock_6::*;
#[cfg(feature = "clock-7")]
pub use clock_7::*;
#[cfg(feature = "clock-8")]
pub use clock_8::*;
#[cfg(feature = "clock-9")]
pub use clock_9::*;
#[cfg(feature = "clock-alert")]
pub use clock_alert::*;
#[cfg(feature = "clock-arrow-down")]
pub use clock_arrow_down::*;
#[cfg(feature = "clock-arrow-up")]
pub use clock_arrow_up::*;
#[cfg(feature = "cloud")]
pub use cloud::*;
#[cfg(feature = "cloud-alert")]
pub use cloud_alert::*;
#[cfg(feature = "cloud-cog")]
pub use cloud_cog::*;
#[cfg(feature = "cloud-download")]
pub use cloud_download::*;
#[cfg(feature = "cloud-drizzle")]
pub use cloud_drizzle::*;
#[cfg(feature = "cloud-fog")]
pub use cloud_fog::*;
#[cfg(feature = "cloud-hail")]
pub use cloud_hail::*;
#[cfg(feature = "cloud-lightning")]
pub use cloud_lightning::*;
#[cfg(feature = "cloud-moon")]
pub use cloud_moon::*;
#[cfg(feature = "cloud-moon-rain")]
pub use cloud_moon_rain::*;
#[cfg(feature = "cloud-off")]
pub use cloud_off::*;
#[cfg(feature = "cloud-rain")]
pub use cloud_rain::*;
#[cfg(feature = "cloud-rain-wind")]
pub use cloud_rain_wind::*;
#[cfg(feature = "cloud-snow")]
pub use cloud_snow::*;
#[cfg(feature = "cloud-sun")]
pub use cloud_sun::*;
#[cfg(feature = "cloud-sun-rain")]
pub use cloud_sun_rain::*;
#[cfg(feature = "cloud-upload")]
pub use cloud_upload::*;
#[cfg(feature = "cloudy")]
pub use cloudy::*;
#[cfg(feature = "clover")]
pub use clover::*;
#[cfg(feature = "club")]
pub use club::*;
#[cfg(feature = "code")]
pub use code::*;
#[cfg(feature = "code-xml")]
pub use code_xml::*;
#[cfg(feature = "codepen")]
pub use codepen::*;
#[cfg(feature = "codesandbox")]
pub use codesandbox::*;
#[cfg(feature = "coffee")]
pub use coffee::*;
#[cfg(feature = "cog")]
pub use cog::*;
#[cfg(feature = "coins")]
pub use coins::*;
#[cfg(feature = "columns-2")]
pub use columns_2::*;
#[cfg(feature = "columns-3")]
pub use columns_3::*;
#[cfg(feature = "columns-4")]
pub use columns_4::*;
#[cfg(feature = "combine")]
pub use combine::*;
#[cfg(feature = "command")]
pub use command::*;
#[cfg(feature = "compass")]
pub use compass::*;
#[cfg(feature = "component")]
pub use component::*;
#[cfg(feature = "computer")]
pub use computer::*;
#[cfg(feature = "concierge-bell")]
pub use concierge_bell::*;
#[cfg(feature = "cone")]
pub use cone::*;
#[cfg(feature = "construction")]
pub use construction::*;
#[cfg(feature = "contact")]
pub use contact::*;
#[cfg(feature = "contact-round")]
pub use contact_round::*;
#[cfg(feature = "container")]
pub use container::*;
#[cfg(feature = "contrast")]
pub use contrast::*;
#[cfg(feature = "cookie")]
pub use cookie::*;
#[cfg(feature = "cooking-pot")]
pub use cooking_pot::*;
#[cfg(feature = "copy")]
pub use copy::*;
#[cfg(feature = "copy-check")]
pub use copy_check::*;
#[cfg(feature = "copy-minus")]
pub use copy_minus::*;
#[cfg(feature = "copy-plus")]
pub use copy_plus::*;
#[cfg(feature = "copy-slash")]
pub use copy_slash::*;
#[cfg(feature = "copy-x")]
pub use copy_x::*;
#[cfg(feature = "copyleft")]
pub use copyleft::*;
#[cfg(feature = "copyright")]
pub use copyright::*;
#[cfg(feature = "corner-down-left")]
pub use corner_down_left::*;
#[cfg(feature = "corner-down-right")]
pub use corner_down_right::*;
#[cfg(feature = "corner-left-down")]
pub use corner_left_down::*;
#[cfg(feature = "corner-left-up")]
pub use corner_left_up::*;
#[cfg(feature = "corner-right-down")]
pub use corner_right_down::*;
#[cfg(feature = "corner-right-up")]
pub use corner_right_up::*;
#[cfg(feature = "corner-up-left")]
pub use corner_up_left::*;
#[cfg(feature = "corner-up-right")]
pub use corner_up_right::*;
#[cfg(feature = "cpu")]
pub use cpu::*;
#[cfg(feature = "creative-commons")]
pub use creative_commons::*;
#[cfg(feature = "credit-card")]
pub use credit_card::*;
#[cfg(feature = "croissant")]
pub use croissant::*;
#[cfg(feature = "crop")]
pub use crop::*;
#[cfg(feature = "cross")]
pub use cross::*;
#[cfg(feature = "crosshair")]
pub use crosshair::*;
#[cfg(feature = "crown")]
pub use crown::*;
#[cfg(feature = "cuboid")]
pub use cuboid::*;
#[cfg(feature = "cup-soda")]
pub use cup_soda::*;
#[cfg(feature = "currency")]
pub use currency::*;
#[cfg(feature = "cylinder")]
pub use cylinder::*;
#[cfg(feature = "dam")]
pub use dam::*;
#[cfg(feature = "database")]
pub use database::*;
#[cfg(feature = "database-backup")]
pub use database_backup::*;
#[cfg(feature = "database-zap")]
pub use database_zap::*;
#[cfg(feature = "delete")]
pub use delete::*;
#[cfg(feature = "dessert")]
pub use dessert::*;
#[cfg(feature = "diameter")]
pub use diameter::*;
#[cfg(feature = "diamond")]
pub use diamond::*;
#[cfg(feature = "diamond-minus")]
pub use diamond_minus::*;
#[cfg(feature = "diamond-percent")]
pub use diamond_percent::*;
#[cfg(feature = "diamond-plus")]
pub use diamond_plus::*;
#[cfg(feature = "dice-1")]
pub use dice_1::*;
#[cfg(feature = "dice-2")]
pub use dice_2::*;
#[cfg(feature = "dice-3")]
pub use dice_3::*;
#[cfg(feature = "dice-4")]
pub use dice_4::*;
#[cfg(feature = "dice-5")]
pub use dice_5::*;
#[cfg(feature = "dice-6")]
pub use dice_6::*;
#[cfg(feature = "dices")]
pub use dices::*;
#[cfg(feature = "diff")]
pub use diff::*;
#[cfg(feature = "disc")]
pub use disc::*;
#[cfg(feature = "disc-2")]
pub use disc_2::*;
#[cfg(feature = "disc-3")]
pub use disc_3::*;
#[cfg(feature = "disc-album")]
pub use disc_album::*;
#[cfg(feature = "divide")]
pub use divide::*;
#[cfg(feature = "dna")]
pub use dna::*;
#[cfg(feature = "dna-off")]
pub use dna_off::*;
#[cfg(feature = "dock")]
pub use dock::*;
#[cfg(feature = "dog")]
pub use dog::*;
#[cfg(feature = "dollar-sign")]
pub use dollar_sign::*;
#[cfg(feature = "donut")]
pub use donut::*;
#[cfg(feature = "door-closed")]
pub use door_closed::*;
#[cfg(feature = "door-open")]
pub use door_open::*;
#[cfg(feature = "dot")]
pub use dot::*;
#[cfg(feature = "download")]
pub use download::*;
#[cfg(feature = "drafting-compass")]
pub use drafting_compass::*;
#[cfg(feature = "drama")]
pub use drama::*;
#[cfg(feature = "dribbble")]
pub use dribbble::*;
#[cfg(feature = "drill")]
pub use drill::*;
#[cfg(feature = "droplet")]
pub use droplet::*;
#[cfg(feature = "droplets")]
pub use droplets::*;
#[cfg(feature = "drum")]
pub use drum::*;
#[cfg(feature = "drumstick")]
pub use drumstick::*;
#[cfg(feature = "dumbbell")]
pub use dumbbell::*;
#[cfg(feature = "ear")]
pub use ear::*;
#[cfg(feature = "ear-off")]
pub use ear_off::*;
#[cfg(feature = "earth")]
pub use earth::*;
#[cfg(feature = "earth-lock")]
pub use earth_lock::*;
#[cfg(feature = "eclipse")]
pub use eclipse::*;
#[cfg(feature = "egg")]
pub use egg::*;
#[cfg(feature = "egg-fried")]
pub use egg_fried::*;
#[cfg(feature = "egg-off")]
pub use egg_off::*;
#[cfg(feature = "ellipsis")]
pub use ellipsis::*;
#[cfg(feature = "ellipsis-vertical")]
pub use ellipsis_vertical::*;
#[cfg(feature = "equal")]
pub use equal::*;
#[cfg(feature = "equal-approximately")]
pub use equal_approximately::*;
#[cfg(feature = "equal-not")]
pub use equal_not::*;
#[cfg(feature = "eraser")]
pub use eraser::*;
#[cfg(feature = "ethernet-port")]
pub use ethernet_port::*;
#[cfg(feature = "euro")]
pub use euro::*;
#[cfg(feature = "expand")]
pub use expand::*;
#[cfg(feature = "external-link")]
pub use external_link::*;
#[cfg(feature = "eye")]
pub use eye::*;
#[cfg(feature = "eye-closed")]
pub use eye_closed::*;
#[cfg(feature = "eye-off")]
pub use eye_off::*;
#[cfg(feature = "facebook")]
pub use facebook::*;
#[cfg(feature = "factory")]
pub use factory::*;
#[cfg(feature = "fan")]
pub use fan::*;
#[cfg(feature = "fast-forward")]
pub use fast_forward::*;
#[cfg(feature = "feather")]
pub use feather::*;
#[cfg(feature = "fence")]
pub use fence::*;
#[cfg(feature = "ferris-wheel")]
pub use ferris_wheel::*;
#[cfg(feature = "figma")]
pub use figma::*;
#[cfg(feature = "file")]
pub use file::*;
#[cfg(feature = "file-archive")]
pub use file_archive::*;
#[cfg(feature = "file-audio")]
pub use file_audio::*;
#[cfg(feature = "file-audio-2")]
pub use file_audio_2::*;
#[cfg(feature = "file-axis-3-d")]
pub use file_axis_3_d::*;
#[cfg(feature = "file-badge")]
pub use file_badge::*;
#[cfg(feature = "file-badge-2")]
pub use file_badge_2::*;
#[cfg(feature = "file-box")]
pub use file_box::*;
#[cfg(feature = "file-chart-column")]
pub use file_chart_column::*;
#[cfg(feature = "file-chart-column-increasing")]
pub use file_chart_column_increasing::*;
#[cfg(feature = "file-chart-line")]
pub use file_chart_line::*;
#[cfg(feature = "file-chart-pie")]
pub use file_chart_pie::*;
#[cfg(feature = "file-check")]
pub use file_check::*;
#[cfg(feature = "file-check-2")]
pub use file_check_2::*;
#[cfg(feature = "file-clock")]
pub use file_clock::*;
#[cfg(feature = "file-code")]
pub use file_code::*;
#[cfg(feature = "file-code-2")]
pub use file_code_2::*;
#[cfg(feature = "file-cog")]
pub use file_cog::*;
#[cfg(feature = "file-diff")]
pub use file_diff::*;
#[cfg(feature = "file-digit")]
pub use file_digit::*;
#[cfg(feature = "file-down")]
pub use file_down::*;
#[cfg(feature = "file-heart")]
pub use file_heart::*;
#[cfg(feature = "file-image")]
pub use file_image::*;
#[cfg(feature = "file-input")]
pub use file_input::*;
#[cfg(feature = "file-json")]
pub use file_json::*;
#[cfg(feature = "file-json-2")]
pub use file_json_2::*;
#[cfg(feature = "file-key")]
pub use file_key::*;
#[cfg(feature = "file-key-2")]
pub use file_key_2::*;
#[cfg(feature = "file-lock")]
pub use file_lock::*;
#[cfg(feature = "file-lock-2")]
pub use file_lock_2::*;
#[cfg(feature = "file-minus")]
pub use file_minus::*;
#[cfg(feature = "file-minus-2")]
pub use file_minus_2::*;
#[cfg(feature = "file-music")]
pub use file_music::*;
#[cfg(feature = "file-output")]
pub use file_output::*;
#[cfg(feature = "file-pen")]
pub use file_pen::*;
#[cfg(feature = "file-pen-line")]
pub use file_pen_line::*;
#[cfg(feature = "file-plus")]
pub use file_plus::*;
#[cfg(feature = "file-plus-2")]
pub use file_plus_2::*;
#[cfg(feature = "file-question")]
pub use file_question::*;
#[cfg(feature = "file-scan")]
pub use file_scan::*;
#[cfg(feature = "file-search")]
pub use file_search::*;
#[cfg(feature = "file-search-2")]
pub use file_search_2::*;
#[cfg(feature = "file-sliders")]
pub use file_sliders::*;
#[cfg(feature = "file-spreadsheet")]
pub use file_spreadsheet::*;
#[cfg(feature = "file-stack")]
pub use file_stack::*;
#[cfg(feature = "file-symlink")]
pub use file_symlink::*;
#[cfg(feature = "file-terminal")]
pub use file_terminal::*;
#[cfg(feature = "file-text")]
pub use file_text::*;
#[cfg(feature = "file-type")]
pub use file_type::*;
#[cfg(feature = "file-type-2")]
pub use file_type_2::*;
#[cfg(feature = "file-up")]
pub use file_up::*;
#[cfg(feature = "file-user")]
pub use file_user::*;
#[cfg(feature = "file-video")]
pub use file_video::*;
#[cfg(feature = "file-video-2")]
pub use file_video_2::*;
#[cfg(feature = "file-volume")]
pub use file_volume::*;
#[cfg(feature = "file-volume-2")]
pub use file_volume_2::*;
#[cfg(feature = "file-warning")]
pub use file_warning::*;
#[cfg(feature = "file-x")]
pub use file_x::*;
#[cfg(feature = "file-x-2")]
pub use file_x_2::*;
#[cfg(feature = "files")]
pub use files::*;
#[cfg(feature = "film")]
pub use film::*;
#[cfg(feature = "filter")]
pub use filter::*;
#[cfg(feature = "filter-x")]
pub use filter_x::*;
#[cfg(feature = "fingerprint")]
pub use fingerprint::*;
#[cfg(feature = "fire-extinguisher")]
pub use fire_extinguisher::*;
#[cfg(feature = "fish")]
pub use fish::*;
#[cfg(feature = "fish-off")]
pub use fish_off::*;
#[cfg(feature = "fish-symbol")]
pub use fish_symbol::*;
#[cfg(feature = "flag")]
pub use flag::*;
#[cfg(feature = "flag-off")]
pub use flag_off::*;
#[cfg(feature = "flag-triangle-left")]
pub use flag_triangle_left::*;
#[cfg(feature = "flag-triangle-right")]
pub use flag_triangle_right::*;
#[cfg(feature = "flame")]
pub use flame::*;
#[cfg(feature = "flame-kindling")]
pub use flame_kindling::*;
#[cfg(feature = "flashlight")]
pub use flashlight::*;
#[cfg(feature = "flashlight-off")]
pub use flashlight_off::*;
#[cfg(feature = "flask-conical")]
pub use flask_conical::*;
#[cfg(feature = "flask-conical-off")]
pub use flask_conical_off::*;
#[cfg(feature = "flask-round")]
pub use flask_round::*;
#[cfg(feature = "flip-horizontal")]
pub use flip_horizontal::*;
#[cfg(feature = "flip-horizontal-2")]
pub use flip_horizontal_2::*;
#[cfg(feature = "flip-vertical")]
pub use flip_vertical::*;
#[cfg(feature = "flip-vertical-2")]
pub use flip_vertical_2::*;
#[cfg(feature = "flower")]
pub use flower::*;
#[cfg(feature = "flower-2")]
pub use flower_2::*;
#[cfg(feature = "focus")]
pub use focus::*;
#[cfg(feature = "fold-horizontal")]
pub use fold_horizontal::*;
#[cfg(feature = "fold-vertical")]
pub use fold_vertical::*;
#[cfg(feature = "folder")]
pub use folder::*;
#[cfg(feature = "folder-archive")]
pub use folder_archive::*;
#[cfg(feature = "folder-check")]
pub use folder_check::*;
#[cfg(feature = "folder-clock")]
pub use folder_clock::*;
#[cfg(feature = "folder-closed")]
pub use folder_closed::*;
#[cfg(feature = "folder-code")]
pub use folder_code::*;
#[cfg(feature = "folder-cog")]
pub use folder_cog::*;
#[cfg(feature = "folder-dot")]
pub use folder_dot::*;
#[cfg(feature = "folder-down")]
pub use folder_down::*;
#[cfg(feature = "folder-git")]
pub use folder_git::*;
#[cfg(feature = "folder-git-2")]
pub use folder_git_2::*;
#[cfg(feature = "folder-heart")]
pub use folder_heart::*;
#[cfg(feature = "folder-input")]
pub use folder_input::*;
#[cfg(feature = "folder-kanban")]
pub use folder_kanban::*;
#[cfg(feature = "folder-key")]
pub use folder_key::*;
#[cfg(feature = "folder-lock")]
pub use folder_lock::*;
#[cfg(feature = "folder-minus")]
pub use folder_minus::*;
#[cfg(feature = "folder-open")]
pub use folder_open::*;
#[cfg(feature = "folder-open-dot")]
pub use folder_open_dot::*;
#[cfg(feature = "folder-output")]
pub use folder_output::*;
#[cfg(feature = "folder-pen")]
pub use folder_pen::*;
#[cfg(feature = "folder-plus")]
pub use folder_plus::*;
#[cfg(feature = "folder-root")]
pub use folder_root::*;
#[cfg(feature = "folder-search")]
pub use folder_search::*;
#[cfg(feature = "folder-search-2")]
pub use folder_search_2::*;
#[cfg(feature = "folder-symlink")]
pub use folder_symlink::*;
#[cfg(feature = "folder-sync")]
pub use folder_sync::*;
#[cfg(feature = "folder-tree")]
pub use folder_tree::*;
#[cfg(feature = "folder-up")]
pub use folder_up::*;
#[cfg(feature = "folder-x")]
pub use folder_x::*;
#[cfg(feature = "folders")]
pub use folders::*;
#[cfg(feature = "footprints")]
pub use footprints::*;
#[cfg(feature = "forklift")]
pub use forklift::*;
#[cfg(feature = "forward")]
pub use forward::*;
#[cfg(feature = "frame")]
pub use frame::*;
#[cfg(feature = "framer")]
pub use framer::*;
#[cfg(feature = "frown")]
pub use frown::*;
#[cfg(feature = "fuel")]
pub use fuel::*;
#[cfg(feature = "fullscreen")]
pub use fullscreen::*;
#[cfg(feature = "gallery-horizontal")]
pub use gallery_horizontal::*;
#[cfg(feature = "gallery-horizontal-end")]
pub use gallery_horizontal_end::*;
#[cfg(feature = "gallery-thumbnails")]
pub use gallery_thumbnails::*;
#[cfg(feature = "gallery-vertical")]
pub use gallery_vertical::*;
#[cfg(feature = "gallery-vertical-end")]
pub use gallery_vertical_end::*;
#[cfg(feature = "gamepad")]
pub use gamepad::*;
#[cfg(feature = "gamepad-2")]
pub use gamepad_2::*;
#[cfg(feature = "gauge")]
pub use gauge::*;
#[cfg(feature = "gavel")]
pub use gavel::*;
#[cfg(feature = "gem")]
pub use gem::*;
#[cfg(feature = "ghost")]
pub use ghost::*;
#[cfg(feature = "gift")]
pub use gift::*;
#[cfg(feature = "git-branch")]
pub use git_branch::*;
#[cfg(feature = "git-branch-plus")]
pub use git_branch_plus::*;
#[cfg(feature = "git-commit-horizontal")]
pub use git_commit_horizontal::*;
#[cfg(feature = "git-commit-vertical")]
pub use git_commit_vertical::*;
#[cfg(feature = "git-compare")]
pub use git_compare::*;
#[cfg(feature = "git-compare-arrows")]
pub use git_compare_arrows::*;
#[cfg(feature = "git-fork")]
pub use git_fork::*;
#[cfg(feature = "git-graph")]
pub use git_graph::*;
#[cfg(feature = "git-merge")]
pub use git_merge::*;
#[cfg(feature = "git-pull-request")]
pub use git_pull_request::*;
#[cfg(feature = "git-pull-request-arrow")]
pub use git_pull_request_arrow::*;
#[cfg(feature = "git-pull-request-closed")]
pub use git_pull_request_closed::*;
#[cfg(feature = "git-pull-request-create")]
pub use git_pull_request_create::*;
#[cfg(feature = "git-pull-request-create-arrow")]
pub use git_pull_request_create_arrow::*;
#[cfg(feature = "git-pull-request-draft")]
pub use git_pull_request_draft::*;
#[cfg(feature = "github")]
pub use github::*;
#[cfg(feature = "gitlab")]
pub use gitlab::*;
#[cfg(feature = "glass-water")]
pub use glass_water::*;
#[cfg(feature = "glasses")]
pub use glasses::*;
#[cfg(feature = "globe")]
pub use globe::*;
#[cfg(feature = "globe-lock")]
pub use globe_lock::*;
#[cfg(feature = "goal")]
pub use goal::*;
#[cfg(feature = "grab")]
pub use grab::*;
#[cfg(feature = "graduation-cap")]
pub use graduation_cap::*;
#[cfg(feature = "grape")]
pub use grape::*;
#[cfg(feature = "grid-2-x-2")]
pub use grid_2_x_2::*;
#[cfg(feature = "grid-2-x-2-check")]
pub use grid_2_x_2_check::*;
#[cfg(feature = "grid-2-x-2-plus")]
pub use grid_2_x_2_plus::*;
#[cfg(feature = "grid-2-x-2-x")]
pub use grid_2_x_2_x::*;
#[cfg(feature = "grid-3-x-3")]
pub use grid_3_x_3::*;
#[cfg(feature = "grip")]
pub use grip::*;
#[cfg(feature = "grip-horizontal")]
pub use grip_horizontal::*;
#[cfg(feature = "grip-vertical")]
pub use grip_vertical::*;
#[cfg(feature = "group")]
pub use group::*;
#[cfg(feature = "guitar")]
pub use guitar::*;
#[cfg(feature = "ham")]
pub use ham::*;
#[cfg(feature = "hammer")]
pub use hammer::*;
#[cfg(feature = "hand")]
pub use hand::*;
#[cfg(feature = "hand-coins")]
pub use hand_coins::*;
#[cfg(feature = "hand-heart")]
pub use hand_heart::*;
#[cfg(feature = "hand-helping")]
pub use hand_helping::*;
#[cfg(feature = "hand-metal")]
pub use hand_metal::*;
#[cfg(feature = "hand-platter")]
pub use hand_platter::*;
#[cfg(feature = "handshake")]
pub use handshake::*;
#[cfg(feature = "hard-drive")]
pub use hard_drive::*;
#[cfg(feature = "hard-drive-download")]
pub use hard_drive_download::*;
#[cfg(feature = "hard-drive-upload")]
pub use hard_drive_upload::*;
#[cfg(feature = "hard-hat")]
pub use hard_hat::*;
#[cfg(feature = "hash")]
pub use hash::*;
#[cfg(feature = "haze")]
pub use haze::*;
#[cfg(feature = "hdmi-port")]
pub use hdmi_port::*;
#[cfg(feature = "heading")]
pub use heading::*;
#[cfg(feature = "heading-1")]
pub use heading_1::*;
#[cfg(feature = "heading-2")]
pub use heading_2::*;
#[cfg(feature = "heading-3")]
pub use heading_3::*;
#[cfg(feature = "heading-4")]
pub use heading_4::*;
#[cfg(feature = "heading-5")]
pub use heading_5::*;
#[cfg(feature = "heading-6")]
pub use heading_6::*;
#[cfg(feature = "headphone-off")]
pub use headphone_off::*;
#[cfg(feature = "headphones")]
pub use headphones::*;
#[cfg(feature = "headset")]
pub use headset::*;
#[cfg(feature = "heart")]
pub use heart::*;
#[cfg(feature = "heart-crack")]
pub use heart_crack::*;
#[cfg(feature = "heart-handshake")]
pub use heart_handshake::*;
#[cfg(feature = "heart-off")]
pub use heart_off::*;
#[cfg(feature = "heart-pulse")]
pub use heart_pulse::*;
#[cfg(feature = "heater")]
pub use heater::*;
#[cfg(feature = "hexagon")]
pub use hexagon::*;
#[cfg(feature = "highlighter")]
pub use highlighter::*;
#[cfg(feature = "history")]
pub use history::*;
#[cfg(feature = "hop")]
pub use hop::*;
#[cfg(feature = "hop-off")]
pub use hop_off::*;
#[cfg(feature = "hospital")]
pub use hospital::*;
#[cfg(feature = "hotel")]
pub use hotel::*;
#[cfg(feature = "hourglass")]
pub use hourglass::*;
#[cfg(feature = "house")]
pub use house::*;
#[cfg(feature = "house-plug")]
pub use house_plug::*;
#[cfg(feature = "house-plus")]
pub use house_plus::*;
#[cfg(feature = "ice-cream-bowl")]
pub use ice_cream_bowl::*;
#[cfg(feature = "ice-cream-cone")]
pub use ice_cream_cone::*;
#[cfg(feature = "id-card")]
pub use id_card::*;
#[cfg(feature = "image")]
pub use image::*;
#[cfg(feature = "image-down")]
pub use image_down::*;
#[cfg(feature = "image-minus")]
pub use image_minus::*;
#[cfg(feature = "image-off")]
pub use image_off::*;
#[cfg(feature = "image-play")]
pub use image_play::*;
#[cfg(feature = "image-plus")]
pub use image_plus::*;
#[cfg(feature = "image-up")]
pub use image_up::*;
#[cfg(feature = "images")]
pub use images::*;
#[cfg(feature = "import")]
pub use import::*;
#[cfg(feature = "inbox")]
pub use inbox::*;
#[cfg(feature = "indent-decrease")]
pub use indent_decrease::*;
#[cfg(feature = "indent-increase")]
pub use indent_increase::*;
#[cfg(feature = "indian-rupee")]
pub use indian_rupee::*;
#[cfg(feature = "infinity")]
pub use infinity::*;
#[cfg(feature = "info")]
pub use info::*;
#[cfg(feature = "inspection-panel")]
pub use inspection_panel::*;
#[cfg(feature = "instagram")]
pub use instagram::*;
#[cfg(feature = "italic")]
pub use italic::*;
#[cfg(feature = "iteration-ccw")]
pub use iteration_ccw::*;
#[cfg(feature = "iteration-cw")]
pub use iteration_cw::*;
#[cfg(feature = "japanese-yen")]
pub use japanese_yen::*;
#[cfg(feature = "joystick")]
pub use joystick::*;
#[cfg(feature = "kanban")]
pub use kanban::*;
#[cfg(feature = "key")]
pub use key::*;
#[cfg(feature = "key-round")]
pub use key_round::*;
#[cfg(feature = "key-square")]
pub use key_square::*;
#[cfg(feature = "keyboard")]
pub use keyboard::*;
#[cfg(feature = "keyboard-music")]
pub use keyboard_music::*;
#[cfg(feature = "keyboard-off")]
pub use keyboard_off::*;
#[cfg(feature = "lamp")]
pub use lamp::*;
#[cfg(feature = "lamp-ceiling")]
pub use lamp_ceiling::*;
#[cfg(feature = "lamp-desk")]
pub use lamp_desk::*;
#[cfg(feature = "lamp-floor")]
pub use lamp_floor::*;
#[cfg(feature = "lamp-wall-down")]
pub use lamp_wall_down::*;
#[cfg(feature = "lamp-wall-up")]
pub use lamp_wall_up::*;
#[cfg(feature = "land-plot")]
pub use land_plot::*;
#[cfg(feature = "landmark")]
pub use landmark::*;
#[cfg(feature = "languages")]
pub use languages::*;
#[cfg(feature = "laptop")]
pub use laptop::*;
#[cfg(feature = "laptop-minimal")]
pub use laptop_minimal::*;
#[cfg(feature = "laptop-minimal-check")]
pub use laptop_minimal_check::*;
#[cfg(feature = "lasso")]
pub use lasso::*;
#[cfg(feature = "lasso-select")]
pub use lasso_select::*;
#[cfg(feature = "laugh")]
pub use laugh::*;
#[cfg(feature = "layers")]
pub use layers::*;
#[cfg(feature = "layers-2")]
pub use layers_2::*;
#[cfg(feature = "layers-3")]
pub use layers_3::*;
#[cfg(feature = "layout-dashboard")]
pub use layout_dashboard::*;
#[cfg(feature = "layout-grid")]
pub use layout_grid::*;
#[cfg(feature = "layout-list")]
pub use layout_list::*;
#[cfg(feature = "layout-panel-left")]
pub use layout_panel_left::*;
#[cfg(feature = "layout-panel-top")]
pub use layout_panel_top::*;
#[cfg(feature = "layout-template")]
pub use layout_template::*;
#[cfg(feature = "leaf")]
pub use leaf::*;
#[cfg(feature = "leafy-green")]
pub use leafy_green::*;
#[cfg(feature = "lectern")]
pub use lectern::*;
#[cfg(feature = "letter-text")]
pub use letter_text::*;
#[cfg(feature = "library")]
pub use library::*;
#[cfg(feature = "library-big")]
pub use library_big::*;
#[cfg(feature = "life-buoy")]
pub use life_buoy::*;
#[cfg(feature = "ligature")]
pub use ligature::*;
#[cfg(feature = "lightbulb")]
pub use lightbulb::*;
#[cfg(feature = "lightbulb-off")]
pub use lightbulb_off::*;
#[cfg(feature = "link")]
pub use link::*;
#[cfg(feature = "link-2")]
pub use link_2::*;
#[cfg(feature = "link-2-off")]
pub use link_2_off::*;
#[cfg(feature = "linkedin")]
pub use linkedin::*;
#[cfg(feature = "list")]
pub use list::*;
#[cfg(feature = "list-check")]
pub use list_check::*;
#[cfg(feature = "list-checks")]
pub use list_checks::*;
#[cfg(feature = "list-collapse")]
pub use list_collapse::*;
#[cfg(feature = "list-end")]
pub use list_end::*;
#[cfg(feature = "list-filter")]
pub use list_filter::*;
#[cfg(feature = "list-minus")]
pub use list_minus::*;
#[cfg(feature = "list-music")]
pub use list_music::*;
#[cfg(feature = "list-ordered")]
pub use list_ordered::*;
#[cfg(feature = "list-plus")]
pub use list_plus::*;
#[cfg(feature = "list-restart")]
pub use list_restart::*;
#[cfg(feature = "list-start")]
pub use list_start::*;
#[cfg(feature = "list-todo")]
pub use list_todo::*;
#[cfg(feature = "list-tree")]
pub use list_tree::*;
#[cfg(feature = "list-video")]
pub use list_video::*;
#[cfg(feature = "list-x")]
pub use list_x::*;
#[cfg(feature = "loader")]
pub use loader::*;
#[cfg(feature = "loader-circle")]
pub use loader_circle::*;
#[cfg(feature = "loader-pinwheel")]
pub use loader_pinwheel::*;
#[cfg(feature = "locate")]
pub use locate::*;
#[cfg(feature = "locate-fixed")]
pub use locate_fixed::*;
#[cfg(feature = "locate-off")]
pub use locate_off::*;
#[cfg(feature = "lock")]
pub use lock::*;
#[cfg(feature = "lock-keyhole")]
pub use lock_keyhole::*;
#[cfg(feature = "lock-keyhole-open")]
pub use lock_keyhole_open::*;
#[cfg(feature = "lock-open")]
pub use lock_open::*;
#[cfg(feature = "log-in")]
pub use log_in::*;
#[cfg(feature = "log-out")]
pub use log_out::*;
#[cfg(feature = "logs")]
pub use logs::*;
#[cfg(feature = "lollipop")]
pub use lollipop::*;
#[cfg(feature = "luggage")]
pub use luggage::*;
#[cfg(feature = "magnet")]
pub use magnet::*;
#[cfg(feature = "mail")]
pub use mail::*;
#[cfg(feature = "mail-check")]
pub use mail_check::*;
#[cfg(feature = "mail-minus")]
pub use mail_minus::*;
#[cfg(feature = "mail-open")]
pub use mail_open::*;
#[cfg(feature = "mail-plus")]
pub use mail_plus::*;
#[cfg(feature = "mail-question")]
pub use mail_question::*;
#[cfg(feature = "mail-search")]
pub use mail_search::*;
#[cfg(feature = "mail-warning")]
pub use mail_warning::*;
#[cfg(feature = "mail-x")]
pub use mail_x::*;
#[cfg(feature = "mailbox")]
pub use mailbox::*;
#[cfg(feature = "mails")]
pub use mails::*;
#[cfg(feature = "map")]
pub use map::*;
#[cfg(feature = "map-pin")]
pub use map_pin::*;
#[cfg(feature = "map-pin-check")]
pub use map_pin_check::*;
#[cfg(feature = "map-pin-check-inside")]
pub use map_pin_check_inside::*;
#[cfg(feature = "map-pin-house")]
pub use map_pin_house::*;
#[cfg(feature = "map-pin-minus")]
pub use map_pin_minus::*;
#[cfg(feature = "map-pin-minus-inside")]
pub use map_pin_minus_inside::*;
#[cfg(feature = "map-pin-off")]
pub use map_pin_off::*;
#[cfg(feature = "map-pin-plus")]
pub use map_pin_plus::*;
#[cfg(feature = "map-pin-plus-inside")]
pub use map_pin_plus_inside::*;
#[cfg(feature = "map-pin-x")]
pub use map_pin_x::*;
#[cfg(feature = "map-pin-x-inside")]
pub use map_pin_x_inside::*;
#[cfg(feature = "map-pinned")]
pub use map_pinned::*;
#[cfg(feature = "martini")]
pub use martini::*;
#[cfg(feature = "maximize")]
pub use maximize::*;
#[cfg(feature = "maximize-2")]
pub use maximize_2::*;
#[cfg(feature = "medal")]
pub use medal::*;
#[cfg(feature = "megaphone")]
pub use megaphone::*;
#[cfg(feature = "megaphone-off")]
pub use megaphone_off::*;
#[cfg(feature = "meh")]
pub use meh::*;
#[cfg(feature = "memory-stick")]
pub use memory_stick::*;
#[cfg(feature = "menu")]
pub use menu::*;
#[cfg(feature = "merge")]
pub use merge::*;
#[cfg(feature = "message-circle")]
pub use message_circle::*;
#[cfg(feature = "message-circle-code")]
pub use message_circle_code::*;
#[cfg(feature = "message-circle-dashed")]
pub use message_circle_dashed::*;
#[cfg(feature = "message-circle-heart")]
pub use message_circle_heart::*;
#[cfg(feature = "message-circle-more")]
pub use message_circle_more::*;
#[cfg(feature = "message-circle-off")]
pub use message_circle_off::*;
#[cfg(feature = "message-circle-plus")]
pub use message_circle_plus::*;
#[cfg(feature = "message-circle-question")]
pub use message_circle_question::*;
#[cfg(feature = "message-circle-reply")]
pub use message_circle_reply::*;
#[cfg(feature = "message-circle-warning")]
pub use message_circle_warning::*;
#[cfg(feature = "message-circle-x")]
pub use message_circle_x::*;
#[cfg(feature = "message-square")]
pub use message_square::*;
#[cfg(feature = "message-square-code")]
pub use message_square_code::*;
#[cfg(feature = "message-square-dashed")]
pub use message_square_dashed::*;
#[cfg(feature = "message-square-diff")]
pub use message_square_diff::*;
#[cfg(feature = "message-square-dot")]
pub use message_square_dot::*;
#[cfg(feature = "message-square-heart")]
pub use message_square_heart::*;
#[cfg(feature = "message-square-lock")]
pub use message_square_lock::*;
#[cfg(feature = "message-square-more")]
pub use message_square_more::*;
#[cfg(feature = "message-square-off")]
pub use message_square_off::*;
#[cfg(feature = "message-square-plus")]
pub use message_square_plus::*;
#[cfg(feature = "message-square-quote")]
pub use message_square_quote::*;
#[cfg(feature = "message-square-reply")]
pub use message_square_reply::*;
#[cfg(feature = "message-square-share")]
pub use message_square_share::*;
#[cfg(feature = "message-square-text")]
pub use message_square_text::*;
#[cfg(feature = "message-square-warning")]
pub use message_square_warning::*;
#[cfg(feature = "message-square-x")]
pub use message_square_x::*;
#[cfg(feature = "messages-square")]
pub use messages_square::*;
#[cfg(feature = "mic")]
pub use mic::*;
#[cfg(feature = "mic-off")]
pub use mic_off::*;
#[cfg(feature = "mic-vocal")]
pub use mic_vocal::*;
#[cfg(feature = "microchip")]
pub use microchip::*;
#[cfg(feature = "microscope")]
pub use microscope::*;
#[cfg(feature = "microwave")]
pub use microwave::*;
#[cfg(feature = "milestone")]
pub use milestone::*;
#[cfg(feature = "milk")]
pub use milk::*;
#[cfg(feature = "milk-off")]
pub use milk_off::*;
#[cfg(feature = "minimize")]
pub use minimize::*;
#[cfg(feature = "minimize-2")]
pub use minimize_2::*;
#[cfg(feature = "minus")]
pub use minus::*;
#[cfg(feature = "monitor")]
pub use monitor::*;
#[cfg(feature = "monitor-check")]
pub use monitor_check::*;
#[cfg(feature = "monitor-cog")]
pub use monitor_cog::*;
#[cfg(feature = "monitor-dot")]
pub use monitor_dot::*;
#[cfg(feature = "monitor-down")]
pub use monitor_down::*;
#[cfg(feature = "monitor-off")]
pub use monitor_off::*;
#[cfg(feature = "monitor-pause")]
pub use monitor_pause::*;
#[cfg(feature = "monitor-play")]
pub use monitor_play::*;
#[cfg(feature = "monitor-smartphone")]
pub use monitor_smartphone::*;
#[cfg(feature = "monitor-speaker")]
pub use monitor_speaker::*;
#[cfg(feature = "monitor-stop")]
pub use monitor_stop::*;
#[cfg(feature = "monitor-up")]
pub use monitor_up::*;
#[cfg(feature = "monitor-x")]
pub use monitor_x::*;
#[cfg(feature = "moon")]
pub use moon::*;
#[cfg(feature = "moon-star")]
pub use moon_star::*;
#[cfg(feature = "mountain")]
pub use mountain::*;
#[cfg(feature = "mountain-snow")]
pub use mountain_snow::*;
#[cfg(feature = "mouse")]
pub use mouse::*;
#[cfg(feature = "mouse-off")]
pub use mouse_off::*;
#[cfg(feature = "mouse-pointer")]
pub use mouse_pointer::*;
#[cfg(feature = "mouse-pointer-2")]
pub use mouse_pointer_2::*;
#[cfg(feature = "mouse-pointer-ban")]
pub use mouse_pointer_ban::*;
#[cfg(feature = "mouse-pointer-click")]
pub use mouse_pointer_click::*;
#[cfg(feature = "move-3-d")]
pub use move_3_d::*;
#[cfg(feature = "move-diagonal")]
pub use move_diagonal::*;
#[cfg(feature = "move-diagonal-2")]
pub use move_diagonal_2::*;
#[cfg(feature = "move-down")]
pub use move_down::*;
#[cfg(feature = "move-down-left")]
pub use move_down_left::*;
#[cfg(feature = "move-down-right")]
pub use move_down_right::*;
#[cfg(feature = "move-horizontal")]
pub use move_horizontal::*;
#[cfg(feature = "move-left")]
pub use move_left::*;
#[cfg(feature = "move-right")]
pub use move_right::*;
#[cfg(feature = "move-up")]
pub use move_up::*;
#[cfg(feature = "move-up-left")]
pub use move_up_left::*;
#[cfg(feature = "move-up-right")]
pub use move_up_right::*;
#[cfg(feature = "move-vertical")]
pub use move_vertical::*;
#[cfg(feature = "music")]
pub use music::*;
#[cfg(feature = "music-2")]
pub use music_2::*;
#[cfg(feature = "music-3")]
pub use music_3::*;
#[cfg(feature = "music-4")]
pub use music_4::*;
#[cfg(feature = "navigation")]
pub use navigation::*;
#[cfg(feature = "navigation-2")]
pub use navigation_2::*;
#[cfg(feature = "navigation-2-off")]
pub use navigation_2_off::*;
#[cfg(feature = "navigation-off")]
pub use navigation_off::*;
#[cfg(feature = "network")]
pub use network::*;
#[cfg(feature = "newspaper")]
pub use newspaper::*;
#[cfg(feature = "nfc")]
pub use nfc::*;
#[cfg(feature = "notebook")]
pub use notebook::*;
#[cfg(feature = "notebook-pen")]
pub use notebook_pen::*;
#[cfg(feature = "notebook-tabs")]
pub use notebook_tabs::*;
#[cfg(feature = "notebook-text")]
pub use notebook_text::*;
#[cfg(feature = "notepad-text")]
pub use notepad_text::*;
#[cfg(feature = "notepad-text-dashed")]
pub use notepad_text_dashed::*;
#[cfg(feature = "nut")]
pub use nut::*;
#[cfg(feature = "nut-off")]
pub use nut_off::*;
#[cfg(feature = "octagon")]
pub use octagon::*;
#[cfg(feature = "octagon-alert")]
pub use octagon_alert::*;
#[cfg(feature = "octagon-minus")]
pub use octagon_minus::*;
#[cfg(feature = "octagon-pause")]
pub use octagon_pause::*;
#[cfg(feature = "octagon-x")]
pub use octagon_x::*;
#[cfg(feature = "omega")]
pub use omega::*;
#[cfg(feature = "option")]
pub use option::*;
#[cfg(feature = "orbit")]
pub use orbit::*;
#[cfg(feature = "origami")]
pub use origami::*;
#[cfg(feature = "package")]
pub use package::*;
#[cfg(feature = "package-2")]
pub use package_2::*;
#[cfg(feature = "package-check")]
pub use package_check::*;
#[cfg(feature = "package-minus")]
pub use package_minus::*;
#[cfg(feature = "package-open")]
pub use package_open::*;
#[cfg(feature = "package-plus")]
pub use package_plus::*;
#[cfg(feature = "package-search")]
pub use package_search::*;
#[cfg(feature = "package-x")]
pub use package_x::*;
#[cfg(feature = "paint-bucket")]
pub use paint_bucket::*;
#[cfg(feature = "paint-roller")]
pub use paint_roller::*;
#[cfg(feature = "paintbrush")]
pub use paintbrush::*;
#[cfg(feature = "paintbrush-vertical")]
pub use paintbrush_vertical::*;
#[cfg(feature = "palette")]
pub use palette::*;
#[cfg(feature = "panel-bottom")]
pub use panel_bottom::*;
#[cfg(feature = "panel-bottom-close")]
pub use panel_bottom_close::*;
#[cfg(feature = "panel-bottom-dashed")]
pub use panel_bottom_dashed::*;
#[cfg(feature = "panel-bottom-open")]
pub use panel_bottom_open::*;
#[cfg(feature = "panel-left")]
pub use panel_left::*;
#[cfg(feature = "panel-left-close")]
pub use panel_left_close::*;
#[cfg(feature = "panel-left-dashed")]
pub use panel_left_dashed::*;
#[cfg(feature = "panel-left-open")]
pub use panel_left_open::*;
#[cfg(feature = "panel-right")]
pub use panel_right::*;
#[cfg(feature = "panel-right-close")]
pub use panel_right_close::*;
#[cfg(feature = "panel-right-dashed")]
pub use panel_right_dashed::*;
#[cfg(feature = "panel-right-open")]
pub use panel_right_open::*;
#[cfg(feature = "panel-top")]
pub use panel_top::*;
#[cfg(feature = "panel-top-close")]
pub use panel_top_close::*;
#[cfg(feature = "panel-top-dashed")]
pub use panel_top_dashed::*;
#[cfg(feature = "panel-top-open")]
pub use panel_top_open::*;
#[cfg(feature = "panels-left-bottom")]
pub use panels_left_bottom::*;
#[cfg(feature = "panels-right-bottom")]
pub use panels_right_bottom::*;
#[cfg(feature = "panels-top-left")]
pub use panels_top_left::*;
#[cfg(feature = "paperclip")]
pub use paperclip::*;
#[cfg(feature = "parentheses")]
pub use parentheses::*;
#[cfg(feature = "parking-meter")]
pub use parking_meter::*;
#[cfg(feature = "party-popper")]
pub use party_popper::*;
#[cfg(feature = "pause")]
pub use pause::*;
#[cfg(feature = "paw-print")]
pub use paw_print::*;
#[cfg(feature = "pc-case")]
pub use pc_case::*;
#[cfg(feature = "pen")]
pub use pen::*;
#[cfg(feature = "pen-line")]
pub use pen_line::*;
#[cfg(feature = "pen-off")]
pub use pen_off::*;
#[cfg(feature = "pen-tool")]
pub use pen_tool::*;
#[cfg(feature = "pencil")]
pub use pencil::*;
#[cfg(feature = "pencil-line")]
pub use pencil_line::*;
#[cfg(feature = "pencil-off")]
pub use pencil_off::*;
#[cfg(feature = "pencil-ruler")]
pub use pencil_ruler::*;
#[cfg(feature = "pentagon")]
pub use pentagon::*;
#[cfg(feature = "percent")]
pub use percent::*;
#[cfg(feature = "person-standing")]
pub use person_standing::*;
#[cfg(feature = "philippine-peso")]
pub use philippine_peso::*;
#[cfg(feature = "phone")]
pub use phone::*;
#[cfg(feature = "phone-call")]
pub use phone_call::*;
#[cfg(feature = "phone-forwarded")]
pub use phone_forwarded::*;
#[cfg(feature = "phone-incoming")]
pub use phone_incoming::*;
#[cfg(feature = "phone-missed")]
pub use phone_missed::*;
#[cfg(feature = "phone-off")]
pub use phone_off::*;
#[cfg(feature = "phone-outgoing")]
pub use phone_outgoing::*;
#[cfg(feature = "pi")]
pub use pi::*;
#[cfg(feature = "piano")]
pub use piano::*;
#[cfg(feature = "pickaxe")]
pub use pickaxe::*;
#[cfg(feature = "picture-in-picture")]
pub use picture_in_picture::*;
#[cfg(feature = "picture-in-picture-2")]
pub use picture_in_picture_2::*;
#[cfg(feature = "piggy-bank")]
pub use piggy_bank::*;
#[cfg(feature = "pilcrow")]
pub use pilcrow::*;
#[cfg(feature = "pilcrow-left")]
pub use pilcrow_left::*;
#[cfg(feature = "pilcrow-right")]
pub use pilcrow_right::*;
#[cfg(feature = "pill")]
pub use pill::*;
#[cfg(feature = "pill-bottle")]
pub use pill_bottle::*;
#[cfg(feature = "pin")]
pub use pin::*;
#[cfg(feature = "pin-off")]
pub use pin_off::*;
#[cfg(feature = "pipette")]
pub use pipette::*;
#[cfg(feature = "pizza")]
pub use pizza::*;
#[cfg(feature = "plane")]
pub use plane::*;
#[cfg(feature = "plane-landing")]
pub use plane_landing::*;
#[cfg(feature = "plane-takeoff")]
pub use plane_takeoff::*;
#[cfg(feature = "play")]
pub use play::*;
#[cfg(feature = "plug")]
pub use plug::*;
#[cfg(feature = "plug-2")]
pub use plug_2::*;
#[cfg(feature = "plug-zap")]
pub use plug_zap::*;
#[cfg(feature = "plus")]
pub use plus::*;
#[cfg(feature = "pocket")]
pub use pocket::*;
#[cfg(feature = "pocket-knife")]
pub use pocket_knife::*;
#[cfg(feature = "podcast")]
pub use podcast::*;
#[cfg(feature = "pointer")]
pub use pointer::*;
#[cfg(feature = "pointer-off")]
pub use pointer_off::*;
#[cfg(feature = "popcorn")]
pub use popcorn::*;
#[cfg(feature = "popsicle")]
pub use popsicle::*;
#[cfg(feature = "pound-sterling")]
pub use pound_sterling::*;
#[cfg(feature = "power")]
pub use power::*;
#[cfg(feature = "power-off")]
pub use power_off::*;
#[cfg(feature = "presentation")]
pub use presentation::*;
#[cfg(feature = "printer")]
pub use printer::*;
#[cfg(feature = "printer-check")]
pub use printer_check::*;
#[cfg(feature = "projector")]
pub use projector::*;
#[cfg(feature = "proportions")]
pub use proportions::*;
#[cfg(feature = "puzzle")]
pub use puzzle::*;
#[cfg(feature = "pyramid")]
pub use pyramid::*;
#[cfg(feature = "qr-code")]
pub use qr_code::*;
#[cfg(feature = "quote")]
pub use quote::*;
#[cfg(feature = "box")]
pub use r#box::*;
#[cfg(feature = "move")]
pub use r#move::*;
#[cfg(feature = "type")]
pub use r#type::*;
#[cfg(feature = "rabbit")]
pub use rabbit::*;
#[cfg(feature = "radar")]
pub use radar::*;
#[cfg(feature = "radiation")]
pub use radiation::*;
#[cfg(feature = "radical")]
pub use radical::*;
#[cfg(feature = "radio")]
pub use radio::*;
#[cfg(feature = "radio-receiver")]
pub use radio_receiver::*;
#[cfg(feature = "radio-tower")]
pub use radio_tower::*;
#[cfg(feature = "radius")]
pub use radius::*;
#[cfg(feature = "rail-symbol")]
pub use rail_symbol::*;
#[cfg(feature = "rainbow")]
pub use rainbow::*;
#[cfg(feature = "rat")]
pub use rat::*;
#[cfg(feature = "ratio")]
pub use ratio::*;
#[cfg(feature = "receipt")]
pub use receipt::*;
#[cfg(feature = "receipt-cent")]
pub use receipt_cent::*;
#[cfg(feature = "receipt-euro")]
pub use receipt_euro::*;
#[cfg(feature = "receipt-indian-rupee")]
pub use receipt_indian_rupee::*;
#[cfg(feature = "receipt-japanese-yen")]
pub use receipt_japanese_yen::*;
#[cfg(feature = "receipt-pound-sterling")]
pub use receipt_pound_sterling::*;
#[cfg(feature = "receipt-russian-ruble")]
pub use receipt_russian_ruble::*;
#[cfg(feature = "receipt-swiss-franc")]
pub use receipt_swiss_franc::*;
#[cfg(feature = "receipt-text")]
pub use receipt_text::*;
#[cfg(feature = "rectangle-ellipsis")]
pub use rectangle_ellipsis::*;
#[cfg(feature = "rectangle-horizontal")]
pub use rectangle_horizontal::*;
#[cfg(feature = "rectangle-vertical")]
pub use rectangle_vertical::*;
#[cfg(feature = "recycle")]
pub use recycle::*;
#[cfg(feature = "redo")]
pub use redo::*;
#[cfg(feature = "redo-2")]
pub use redo_2::*;
#[cfg(feature = "redo-dot")]
pub use redo_dot::*;
#[cfg(feature = "refresh-ccw")]
pub use refresh_ccw::*;
#[cfg(feature = "refresh-ccw-dot")]
pub use refresh_ccw_dot::*;
#[cfg(feature = "refresh-cw")]
pub use refresh_cw::*;
#[cfg(feature = "refresh-cw-off")]
pub use refresh_cw_off::*;
#[cfg(feature = "refrigerator")]
pub use refrigerator::*;
#[cfg(feature = "regex")]
pub use regex::*;
#[cfg(feature = "remove-formatting")]
pub use remove_formatting::*;
#[cfg(feature = "repeat")]
pub use repeat::*;
#[cfg(feature = "repeat-1")]
pub use repeat_1::*;
#[cfg(feature = "repeat-2")]
pub use repeat_2::*;
#[cfg(feature = "replace")]
pub use replace::*;
#[cfg(feature = "replace-all")]
pub use replace_all::*;
#[cfg(feature = "reply")]
pub use reply::*;
#[cfg(feature = "reply-all")]
pub use reply_all::*;
#[cfg(feature = "rewind")]
pub use rewind::*;
#[cfg(feature = "ribbon")]
pub use ribbon::*;
#[cfg(feature = "rocket")]
pub use rocket::*;
#[cfg(feature = "rocking-chair")]
pub use rocking_chair::*;
#[cfg(feature = "roller-coaster")]
pub use roller_coaster::*;
#[cfg(feature = "rotate-3-d")]
pub use rotate_3_d::*;
#[cfg(feature = "rotate-ccw")]
pub use rotate_ccw::*;
#[cfg(feature = "rotate-ccw-square")]
pub use rotate_ccw_square::*;
#[cfg(feature = "rotate-cw")]
pub use rotate_cw::*;
#[cfg(feature = "rotate-cw-square")]
pub use rotate_cw_square::*;
#[cfg(feature = "route")]
pub use route::*;
#[cfg(feature = "route-off")]
pub use route_off::*;
#[cfg(feature = "router")]
pub use router::*;
#[cfg(feature = "rows-2")]
pub use rows_2::*;
#[cfg(feature = "rows-3")]
pub use rows_3::*;
#[cfg(feature = "rows-4")]
pub use rows_4::*;
#[cfg(feature = "rss")]
pub use rss::*;
#[cfg(feature = "ruler")]
pub use ruler::*;
#[cfg(feature = "russian-ruble")]
pub use russian_ruble::*;
#[cfg(feature = "sailboat")]
pub use sailboat::*;
#[cfg(feature = "salad")]
pub use salad::*;
#[cfg(feature = "sandwich")]
pub use sandwich::*;
#[cfg(feature = "satellite")]
pub use satellite::*;
#[cfg(feature = "satellite-dish")]
pub use satellite_dish::*;
#[cfg(feature = "save")]
pub use save::*;
#[cfg(feature = "save-all")]
pub use save_all::*;
#[cfg(feature = "save-off")]
pub use save_off::*;
#[cfg(feature = "scale")]
pub use scale::*;
#[cfg(feature = "scale-3-d")]
pub use scale_3_d::*;
#[cfg(feature = "scaling")]
pub use scaling::*;
#[cfg(feature = "scan")]
pub use scan::*;
#[cfg(feature = "scan-barcode")]
pub use scan_barcode::*;
#[cfg(feature = "scan-eye")]
pub use scan_eye::*;
#[cfg(feature = "scan-face")]
pub use scan_face::*;
#[cfg(feature = "scan-line")]
pub use scan_line::*;
#[cfg(feature = "scan-qr-code")]
pub use scan_qr_code::*;
#[cfg(feature = "scan-search")]
pub use scan_search::*;
#[cfg(feature = "scan-text")]
pub use scan_text::*;
#[cfg(feature = "school")]
pub use school::*;
#[cfg(feature = "scissors")]
pub use scissors::*;
#[cfg(feature = "scissors-line-dashed")]
pub use scissors_line_dashed::*;
#[cfg(feature = "screen-share")]
pub use screen_share::*;
#[cfg(feature = "screen-share-off")]
pub use screen_share_off::*;
#[cfg(feature = "scroll")]
pub use scroll::*;
#[cfg(feature = "scroll-text")]
pub use scroll_text::*;
#[cfg(feature = "search")]
pub use search::*;
#[cfg(feature = "search-check")]
pub use search_check::*;
#[cfg(feature = "search-code")]
pub use search_code::*;
#[cfg(feature = "search-slash")]
pub use search_slash::*;
#[cfg(feature = "search-x")]
pub use search_x::*;
#[cfg(feature = "section")]
pub use section::*;
#[cfg(feature = "send")]
pub use send::*;
#[cfg(feature = "send-horizontal")]
pub use send_horizontal::*;
#[cfg(feature = "send-to-back")]
pub use send_to_back::*;
#[cfg(feature = "separator-horizontal")]
pub use separator_horizontal::*;
#[cfg(feature = "separator-vertical")]
pub use separator_vertical::*;
#[cfg(feature = "server")]
pub use server::*;
#[cfg(feature = "server-cog")]
pub use server_cog::*;
#[cfg(feature = "server-crash")]
pub use server_crash::*;
#[cfg(feature = "server-off")]
pub use server_off::*;
#[cfg(feature = "settings")]
pub use settings::*;
#[cfg(feature = "settings-2")]
pub use settings_2::*;
#[cfg(feature = "shapes")]
pub use shapes::*;
#[cfg(feature = "share")]
pub use share::*;
#[cfg(feature = "share-2")]
pub use share_2::*;
#[cfg(feature = "sheet")]
pub use sheet::*;
#[cfg(feature = "shell")]
pub use shell::*;
#[cfg(feature = "shield")]
pub use shield::*;
#[cfg(feature = "shield-alert")]
pub use shield_alert::*;
#[cfg(feature = "shield-ban")]
pub use shield_ban::*;
#[cfg(feature = "shield-check")]
pub use shield_check::*;
#[cfg(feature = "shield-ellipsis")]
pub use shield_ellipsis::*;
#[cfg(feature = "shield-half")]
pub use shield_half::*;
#[cfg(feature = "shield-minus")]
pub use shield_minus::*;
#[cfg(feature = "shield-off")]
pub use shield_off::*;
#[cfg(feature = "shield-plus")]
pub use shield_plus::*;
#[cfg(feature = "shield-question")]
pub use shield_question::*;
#[cfg(feature = "shield-x")]
pub use shield_x::*;
#[cfg(feature = "ship")]
pub use ship::*;
#[cfg(feature = "ship-wheel")]
pub use ship_wheel::*;
#[cfg(feature = "shirt")]
pub use shirt::*;
#[cfg(feature = "shopping-bag")]
pub use shopping_bag::*;
#[cfg(feature = "shopping-basket")]
pub use shopping_basket::*;
#[cfg(feature = "shopping-cart")]
pub use shopping_cart::*;
#[cfg(feature = "shovel")]
pub use shovel::*;
#[cfg(feature = "shower-head")]
pub use shower_head::*;
#[cfg(feature = "shrink")]
pub use shrink::*;
#[cfg(feature = "shrub")]
pub use shrub::*;
#[cfg(feature = "shuffle")]
pub use shuffle::*;
#[cfg(feature = "sigma")]
pub use sigma::*;
#[cfg(feature = "signal")]
pub use signal::*;
#[cfg(feature = "signal-high")]
pub use signal_high::*;
#[cfg(feature = "signal-low")]
pub use signal_low::*;
#[cfg(feature = "signal-medium")]
pub use signal_medium::*;
#[cfg(feature = "signal-zero")]
pub use signal_zero::*;
#[cfg(feature = "signature")]
pub use signature::*;
#[cfg(feature = "signpost")]
pub use signpost::*;
#[cfg(feature = "signpost-big")]
pub use signpost_big::*;
#[cfg(feature = "siren")]
pub use siren::*;
#[cfg(feature = "skip-back")]
pub use skip_back::*;
#[cfg(feature = "skip-forward")]
pub use skip_forward::*;
#[cfg(feature = "skull")]
pub use skull::*;
#[cfg(feature = "slack")]
pub use slack::*;
#[cfg(feature = "slash")]
pub use slash::*;
#[cfg(feature = "slice")]
pub use slice::*;
#[cfg(feature = "sliders-horizontal")]
pub use sliders_horizontal::*;
#[cfg(feature = "sliders-vertical")]
pub use sliders_vertical::*;
#[cfg(feature = "smartphone")]
pub use smartphone::*;
#[cfg(feature = "smartphone-charging")]
pub use smartphone_charging::*;
#[cfg(feature = "smartphone-nfc")]
pub use smartphone_nfc::*;
#[cfg(feature = "smile")]
pub use smile::*;
#[cfg(feature = "smile-plus")]
pub use smile_plus::*;
#[cfg(feature = "snail")]
pub use snail::*;
#[cfg(feature = "snowflake")]
pub use snowflake::*;
#[cfg(feature = "sofa")]
pub use sofa::*;
#[cfg(feature = "soup")]
pub use soup::*;
#[cfg(feature = "space")]
pub use space::*;
#[cfg(feature = "spade")]
pub use spade::*;
#[cfg(feature = "sparkle")]
pub use sparkle::*;
#[cfg(feature = "sparkles")]
pub use sparkles::*;
#[cfg(feature = "speaker")]
pub use speaker::*;
#[cfg(feature = "speech")]
pub use speech::*;
#[cfg(feature = "spell-check")]
pub use spell_check::*;
#[cfg(feature = "spell-check-2")]
pub use spell_check_2::*;
#[cfg(feature = "spline")]
pub use spline::*;
#[cfg(feature = "split")]
pub use split::*;
#[cfg(feature = "spray-can")]
pub use spray_can::*;
#[cfg(feature = "sprout")]
pub use sprout::*;
#[cfg(feature = "square")]
pub use square::*;
#[cfg(feature = "square-activity")]
pub use square_activity::*;
#[cfg(feature = "square-arrow-down")]
pub use square_arrow_down::*;
#[cfg(feature = "square-arrow-down-left")]
pub use square_arrow_down_left::*;
#[cfg(feature = "square-arrow-down-right")]
pub use square_arrow_down_right::*;
#[cfg(feature = "square-arrow-left")]
pub use square_arrow_left::*;
#[cfg(feature = "square-arrow-out-down-left")]
pub use square_arrow_out_down_left::*;
#[cfg(feature = "square-arrow-out-down-right")]
pub use square_arrow_out_down_right::*;
#[cfg(feature = "square-arrow-out-up-left")]
pub use square_arrow_out_up_left::*;
#[cfg(feature = "square-arrow-out-up-right")]
pub use square_arrow_out_up_right::*;
#[cfg(feature = "square-arrow-right")]
pub use square_arrow_right::*;
#[cfg(feature = "square-arrow-up")]
pub use square_arrow_up::*;
#[cfg(feature = "square-arrow-up-left")]
pub use square_arrow_up_left::*;
#[cfg(feature = "square-arrow-up-right")]
pub use square_arrow_up_right::*;
#[cfg(feature = "square-asterisk")]
pub use square_asterisk::*;
#[cfg(feature = "square-bottom-dashed-scissors")]
pub use square_bottom_dashed_scissors::*;
#[cfg(feature = "square-chart-gantt")]
pub use square_chart_gantt::*;
#[cfg(feature = "square-check")]
pub use square_check::*;
#[cfg(feature = "square-check-big")]
pub use square_check_big::*;
#[cfg(feature = "square-chevron-down")]
pub use square_chevron_down::*;
#[cfg(feature = "square-chevron-left")]
pub use square_chevron_left::*;
#[cfg(feature = "square-chevron-right")]
pub use square_chevron_right::*;
#[cfg(feature = "square-chevron-up")]
pub use square_chevron_up::*;
#[cfg(feature = "square-code")]
pub use square_code::*;
#[cfg(feature = "square-dashed")]
pub use square_dashed::*;
#[cfg(feature = "square-dashed-bottom")]
pub use square_dashed_bottom::*;
#[cfg(feature = "square-dashed-bottom-code")]
pub use square_dashed_bottom_code::*;
#[cfg(feature = "square-dashed-kanban")]
pub use square_dashed_kanban::*;
#[cfg(feature = "square-dashed-mouse-pointer")]
pub use square_dashed_mouse_pointer::*;
#[cfg(feature = "square-divide")]
pub use square_divide::*;
#[cfg(feature = "square-dot")]
pub use square_dot::*;
#[cfg(feature = "square-equal")]
pub use square_equal::*;
#[cfg(feature = "square-function")]
pub use square_function::*;
#[cfg(feature = "square-kanban")]
pub use square_kanban::*;
#[cfg(feature = "square-library")]
pub use square_library::*;
#[cfg(feature = "square-m")]
pub use square_m::*;
#[cfg(feature = "square-menu")]
pub use square_menu::*;
#[cfg(feature = "square-minus")]
pub use square_minus::*;
#[cfg(feature = "square-mouse-pointer")]
pub use square_mouse_pointer::*;
#[cfg(feature = "square-parking")]
pub use square_parking::*;
#[cfg(feature = "square-parking-off")]
pub use square_parking_off::*;
#[cfg(feature = "square-pen")]
pub use square_pen::*;
#[cfg(feature = "square-percent")]
pub use square_percent::*;
#[cfg(feature = "square-pi")]
pub use square_pi::*;
#[cfg(feature = "square-pilcrow")]
pub use square_pilcrow::*;
#[cfg(feature = "square-play")]
pub use square_play::*;
#[cfg(feature = "square-plus")]
pub use square_plus::*;
#[cfg(feature = "square-power")]
pub use square_power::*;
#[cfg(feature = "square-radical")]
pub use square_radical::*;
#[cfg(feature = "square-scissors")]
pub use square_scissors::*;
#[cfg(feature = "square-sigma")]
pub use square_sigma::*;
#[cfg(feature = "square-slash")]
pub use square_slash::*;
#[cfg(feature = "square-split-horizontal")]
pub use square_split_horizontal::*;
#[cfg(feature = "square-split-vertical")]
pub use square_split_vertical::*;
#[cfg(feature = "square-square")]
pub use square_square::*;
#[cfg(feature = "square-stack")]
pub use square_stack::*;
#[cfg(feature = "square-terminal")]
pub use square_terminal::*;
#[cfg(feature = "square-user")]
pub use square_user::*;
#[cfg(feature = "square-user-round")]
pub use square_user_round::*;
#[cfg(feature = "square-x")]
pub use square_x::*;
#[cfg(feature = "squircle")]
pub use squircle::*;
#[cfg(feature = "squirrel")]
pub use squirrel::*;
#[cfg(feature = "stamp")]
pub use stamp::*;
#[cfg(feature = "star")]
pub use star::*;
#[cfg(feature = "star-half")]
pub use star_half::*;
#[cfg(feature = "star-off")]
pub use star_off::*;
#[cfg(feature = "step-back")]
pub use step_back::*;
#[cfg(feature = "step-forward")]
pub use step_forward::*;
#[cfg(feature = "stethoscope")]
pub use stethoscope::*;
#[cfg(feature = "sticker")]
pub use sticker::*;
#[cfg(feature = "sticky-note")]
pub use sticky_note::*;
#[cfg(feature = "store")]
pub use store::*;
#[cfg(feature = "stretch-horizontal")]
pub use stretch_horizontal::*;
#[cfg(feature = "stretch-vertical")]
pub use stretch_vertical::*;
#[cfg(feature = "strikethrough")]
pub use strikethrough::*;
#[cfg(feature = "subscript")]
pub use subscript::*;
#[cfg(feature = "sun")]
pub use sun::*;
#[cfg(feature = "sun-dim")]
pub use sun_dim::*;
#[cfg(feature = "sun-medium")]
pub use sun_medium::*;
#[cfg(feature = "sun-moon")]
pub use sun_moon::*;
#[cfg(feature = "sun-snow")]
pub use sun_snow::*;
#[cfg(feature = "sunrise")]
pub use sunrise::*;
#[cfg(feature = "sunset")]
pub use sunset::*;
#[cfg(feature = "superscript")]
pub use superscript::*;
#[cfg(feature = "swatch-book")]
pub use swatch_book::*;
#[cfg(feature = "swiss-franc")]
pub use swiss_franc::*;
#[cfg(feature = "switch-camera")]
pub use switch_camera::*;
#[cfg(feature = "sword")]
pub use sword::*;
#[cfg(feature = "swords")]
pub use swords::*;
#[cfg(feature = "syringe")]
pub use syringe::*;
#[cfg(feature = "table")]
pub use table::*;
#[cfg(feature = "table-2")]
pub use table_2::*;
#[cfg(feature = "table-cells-merge")]
pub use table_cells_merge::*;
#[cfg(feature = "table-cells-split")]
pub use table_cells_split::*;
#[cfg(feature = "table-columns-split")]
pub use table_columns_split::*;
#[cfg(feature = "table-of-contents")]
pub use table_of_contents::*;
#[cfg(feature = "table-properties")]
pub use table_properties::*;
#[cfg(feature = "table-rows-split")]
pub use table_rows_split::*;
#[cfg(feature = "tablet")]
pub use tablet::*;
#[cfg(feature = "tablet-smartphone")]
pub use tablet_smartphone::*;
#[cfg(feature = "tablets")]
pub use tablets::*;
#[cfg(feature = "tag")]
pub use tag::*;
#[cfg(feature = "tags")]
pub use tags::*;
#[cfg(feature = "tally-1")]
pub use tally_1::*;
#[cfg(feature = "tally-2")]
pub use tally_2::*;
#[cfg(feature = "tally-3")]
pub use tally_3::*;
#[cfg(feature = "tally-4")]
pub use tally_4::*;
#[cfg(feature = "tally-5")]
pub use tally_5::*;
#[cfg(feature = "tangent")]
pub use tangent::*;
#[cfg(feature = "target")]
pub use target::*;
#[cfg(feature = "telescope")]
pub use telescope::*;
#[cfg(feature = "tent")]
pub use tent::*;
#[cfg(feature = "tent-tree")]
pub use tent_tree::*;
#[cfg(feature = "terminal")]
pub use terminal::*;
#[cfg(feature = "test-tube")]
pub use test_tube::*;
#[cfg(feature = "test-tube-diagonal")]
pub use test_tube_diagonal::*;
#[cfg(feature = "test-tubes")]
pub use test_tubes::*;
#[cfg(feature = "text")]
pub use text::*;
#[cfg(feature = "text-cursor")]
pub use text_cursor::*;
#[cfg(feature = "text-cursor-input")]
pub use text_cursor_input::*;
#[cfg(feature = "text-quote")]
pub use text_quote::*;
#[cfg(feature = "text-search")]
pub use text_search::*;
#[cfg(feature = "text-select")]
pub use text_select::*;
#[cfg(feature = "theater")]
pub use theater::*;
#[cfg(feature = "thermometer")]
pub use thermometer::*;
#[cfg(feature = "thermometer-snowflake")]
pub use thermometer_snowflake::*;
#[cfg(feature = "thermometer-sun")]
pub use thermometer_sun::*;
#[cfg(feature = "thumbs-down")]
pub use thumbs_down::*;
#[cfg(feature = "thumbs-up")]
pub use thumbs_up::*;
#[cfg(feature = "ticket")]
pub use ticket::*;
#[cfg(feature = "ticket-check")]
pub use ticket_check::*;
#[cfg(feature = "ticket-minus")]
pub use ticket_minus::*;
#[cfg(feature = "ticket-percent")]
pub use ticket_percent::*;
#[cfg(feature = "ticket-plus")]
pub use ticket_plus::*;
#[cfg(feature = "ticket-slash")]
pub use ticket_slash::*;
#[cfg(feature = "ticket-x")]
pub use ticket_x::*;
#[cfg(feature = "tickets")]
pub use tickets::*;
#[cfg(feature = "tickets-plane")]
pub use tickets_plane::*;
#[cfg(feature = "timer")]
pub use timer::*;
#[cfg(feature = "timer-off")]
pub use timer_off::*;
#[cfg(feature = "timer-reset")]
pub use timer_reset::*;
#[cfg(feature = "toggle-left")]
pub use toggle_left::*;
#[cfg(feature = "toggle-right")]
pub use toggle_right::*;
#[cfg(feature = "toilet")]
pub use toilet::*;
#[cfg(feature = "tornado")]
pub use tornado::*;
#[cfg(feature = "torus")]
pub use torus::*;
#[cfg(feature = "touchpad")]
pub use touchpad::*;
#[cfg(feature = "touchpad-off")]
pub use touchpad_off::*;
#[cfg(feature = "tower-control")]
pub use tower_control::*;
#[cfg(feature = "toy-brick")]
pub use toy_brick::*;
#[cfg(feature = "tractor")]
pub use tractor::*;
#[cfg(feature = "traffic-cone")]
pub use traffic_cone::*;
#[cfg(feature = "train-front")]
pub use train_front::*;
#[cfg(feature = "train-front-tunnel")]
pub use train_front_tunnel::*;
#[cfg(feature = "train-track")]
pub use train_track::*;
#[cfg(feature = "tram-front")]
pub use tram_front::*;
#[cfg(feature = "trash")]
pub use trash::*;
#[cfg(feature = "trash-2")]
pub use trash_2::*;
#[cfg(feature = "tree-deciduous")]
pub use tree_deciduous::*;
#[cfg(feature = "tree-palm")]
pub use tree_palm::*;
#[cfg(feature = "tree-pine")]
pub use tree_pine::*;
#[cfg(feature = "trees")]
pub use trees::*;
#[cfg(feature = "trello")]
pub use trello::*;
#[cfg(feature = "trending-down")]
pub use trending_down::*;
#[cfg(feature = "trending-up")]
pub use trending_up::*;
#[cfg(feature = "trending-up-down")]
pub use trending_up_down::*;
#[cfg(feature = "triangle")]
pub use triangle::*;
#[cfg(feature = "triangle-alert")]
pub use triangle_alert::*;
#[cfg(feature = "triangle-right")]
pub use triangle_right::*;
#[cfg(feature = "trophy")]
pub use trophy::*;
#[cfg(feature = "truck")]
pub use truck::*;
#[cfg(feature = "turtle")]
pub use turtle::*;
#[cfg(feature = "tv")]
pub use tv::*;
#[cfg(feature = "tv-minimal")]
pub use tv_minimal::*;
#[cfg(feature = "tv-minimal-play")]
pub use tv_minimal_play::*;
#[cfg(feature = "twitch")]
pub use twitch::*;
#[cfg(feature = "twitter")]
pub use twitter::*;
#[cfg(feature = "type-outline")]
pub use type_outline::*;
#[cfg(feature = "umbrella")]
pub use umbrella::*;
#[cfg(feature = "umbrella-off")]
pub use umbrella_off::*;
#[cfg(feature = "underline")]
pub use underline::*;
#[cfg(feature = "undo")]
pub use undo::*;
#[cfg(feature = "undo-2")]
pub use undo_2::*;
#[cfg(feature = "undo-dot")]
pub use undo_dot::*;
#[cfg(feature = "unfold-horizontal")]
pub use unfold_horizontal::*;
#[cfg(feature = "unfold-vertical")]
pub use unfold_vertical::*;
#[cfg(feature = "ungroup")]
pub use ungroup::*;
#[cfg(feature = "university")]
pub use university::*;
#[cfg(feature = "unlink")]
pub use unlink::*;
#[cfg(feature = "unlink-2")]
pub use unlink_2::*;
#[cfg(feature = "unplug")]
pub use unplug::*;
#[cfg(feature = "upload")]
pub use upload::*;
#[cfg(feature = "usb")]
pub use usb::*;
#[cfg(feature = "user")]
pub use user::*;
#[cfg(feature = "user-check")]
pub use user_check::*;
#[cfg(feature = "user-cog")]
pub use user_cog::*;
#[cfg(feature = "user-minus")]
pub use user_minus::*;
#[cfg(feature = "user-pen")]
pub use user_pen::*;
#[cfg(feature = "user-plus")]
pub use user_plus::*;
#[cfg(feature = "user-round")]
pub use user_round::*;
#[cfg(feature = "user-round-check")]
pub use user_round_check::*;
#[cfg(feature = "user-round-cog")]
pub use user_round_cog::*;
#[cfg(feature = "user-round-minus")]
pub use user_round_minus::*;
#[cfg(feature = "user-round-pen")]
pub use user_round_pen::*;
#[cfg(feature = "user-round-plus")]
pub use user_round_plus::*;
#[cfg(feature = "user-round-search")]
pub use user_round_search::*;
#[cfg(feature = "user-round-x")]
pub use user_round_x::*;
#[cfg(feature = "user-search")]
pub use user_search::*;
#[cfg(feature = "user-x")]
pub use user_x::*;
#[cfg(feature = "users")]
pub use users::*;
#[cfg(feature = "users-round")]
pub use users_round::*;
#[cfg(feature = "utensils")]
pub use utensils::*;
#[cfg(feature = "utensils-crossed")]
pub use utensils_crossed::*;
#[cfg(feature = "utility-pole")]
pub use utility_pole::*;
#[cfg(feature = "variable")]
pub use variable::*;
#[cfg(feature = "vault")]
pub use vault::*;
#[cfg(feature = "vegan")]
pub use vegan::*;
#[cfg(feature = "venetian-mask")]
pub use venetian_mask::*;
#[cfg(feature = "vibrate")]
pub use vibrate::*;
#[cfg(feature = "vibrate-off")]
pub use vibrate_off::*;
#[cfg(feature = "video")]
pub use video::*;
#[cfg(feature = "video-off")]
pub use video_off::*;
#[cfg(feature = "videotape")]
pub use videotape::*;
#[cfg(feature = "view")]
pub use view::*;
#[cfg(feature = "voicemail")]
pub use voicemail::*;
#[cfg(feature = "volleyball")]
pub use volleyball::*;
#[cfg(feature = "volume")]
pub use volume::*;
#[cfg(feature = "volume-1")]
pub use volume_1::*;
#[cfg(feature = "volume-2")]
pub use volume_2::*;
#[cfg(feature = "volume-off")]
pub use volume_off::*;
#[cfg(feature = "volume-x")]
pub use volume_x::*;
#[cfg(feature = "vote")]
pub use vote::*;
#[cfg(feature = "wallet")]
pub use wallet::*;
#[cfg(feature = "wallet-cards")]
pub use wallet_cards::*;
#[cfg(feature = "wallet-minimal")]
pub use wallet_minimal::*;
#[cfg(feature = "wallpaper")]
pub use wallpaper::*;
#[cfg(feature = "wand")]
pub use wand::*;
#[cfg(feature = "wand-sparkles")]
pub use wand_sparkles::*;
#[cfg(feature = "warehouse")]
pub use warehouse::*;
#[cfg(feature = "washing-machine")]
pub use washing_machine::*;
#[cfg(feature = "watch")]
pub use watch::*;
#[cfg(feature = "waves")]
pub use waves::*;
#[cfg(feature = "waypoints")]
pub use waypoints::*;
#[cfg(feature = "webcam")]
pub use webcam::*;
#[cfg(feature = "webhook")]
pub use webhook::*;
#[cfg(feature = "webhook-off")]
pub use webhook_off::*;
#[cfg(feature = "weight")]
pub use weight::*;
#[cfg(feature = "wheat")]
pub use wheat::*;
#[cfg(feature = "wheat-off")]
pub use wheat_off::*;
#[cfg(feature = "whole-word")]
pub use whole_word::*;
#[cfg(feature = "wifi")]
pub use wifi::*;
#[cfg(feature = "wifi-high")]
pub use wifi_high::*;
#[cfg(feature = "wifi-low")]
pub use wifi_low::*;
#[cfg(feature = "wifi-off")]
pub use wifi_off::*;
#[cfg(feature = "wifi-zero")]
pub use wifi_zero::*;
#[cfg(feature = "wind")]
pub use wind::*;
#[cfg(feature = "wind-arrow-down")]
pub use wind_arrow_down::*;
#[cfg(feature = "wine")]
pub use wine::*;
#[cfg(feature = "wine-off")]
pub use wine_off::*;
#[cfg(feature = "workflow")]
pub use workflow::*;
#[cfg(feature = "worm")]
pub use worm::*;
#[cfg(feature = "wrap-text")]
pub use wrap_text::*;
#[cfg(feature = "wrench")]
pub use wrench::*;
#[cfg(feature = "x")]
pub use x::*;
#[cfg(feature = "youtube")]
pub use youtube::*;
#[cfg(feature = "zap")]
pub use zap::*;
#[cfg(feature = "zap-off")]
pub use zap_off::*;
#[cfg(feature = "zoom-in")]
pub use zoom_in::*;
#[cfg(feature = "zoom-out")]
pub use zoom_out::*;
