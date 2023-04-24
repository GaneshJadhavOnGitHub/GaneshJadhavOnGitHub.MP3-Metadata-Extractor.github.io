/// Program to print MetaData of MP3 file.

use std::env;
use mp3_metadata;
use mp3_metadata::MP3Metadata;

/// Prints Tags in MP3 file.
fn print_tags(mp3_meta_data: &MP3Metadata){
    
    if let Some(tag) = &mp3_meta_data.tag {
        println!("\n\n========== TAGS ==========");
        println!("Title:      {}", tag.title);
        println!("Artist:     {}", tag.artist);
        println!("Album:      {}", tag.album);
        println!("Year:       {}", tag.year);
        println!("Comment:    {}", tag.comment);
        println!("Genre:      {:?}", tag.genre);
    } else {
        println!("\n\nTags are not present.");
    }
}

/// Prints Information of First Frame in MP3 file.
fn print_first_frame(mp3_meta_data: &MP3Metadata){
    println!("\nNumber of Frames: {}", mp3_meta_data.frames.len());
    if mp3_meta_data.frames.len() != 0 {
        println!("\nShowing  First Frame's Information:");
        for frame in mp3_meta_data.frames[0..1].iter() {
            println!("\n========== FIRST FRAME ==========");
            println!("Size:               {}", frame.size);
            println!("Version:            {:?}", frame.version);
            println!("Layer:              {:?}", frame.layer);
            println!("CRC:                {:?}", frame.crc);
            println!("Bitrate:            {} Kb/s", frame.bitrate);
            println!("Sampling Frequency: {} Hz", frame.sampling_freq);
            println!("Padding:            {}", frame.padding);
            println!("Private Bit:        {}", frame.private_bit);
            println!("Channel Type:       {:?}", frame.chan_type);
            println!("Intensity Stereo:   {}", frame.intensity_stereo);
            println!("MS Stereo:          {}", frame.ms_stereo);
            println!("Copyright:          {:?}", frame.copyright);
            println!("Status:             {:?}", frame.status);
            println!("Emphasis:           {:?}", frame.emphasis);
            println!("Duration:           {:?}", frame.duration.to_owned().unwrap());
            println!("Position:           {:?}", frame.position);
            println!("Offset:             {:?}", frame.offset);
        
      } // End of For
    } // End of if
    else {
        println!("\n\nFrames are not present.");
    }
}


/// Prints Optional Information in MP3 file. 
fn print_optional_information(mp3_meta_data: &MP3Metadata){
    
    
    if !mp3_meta_data.optional_info.is_empty(){
        println!("\n========== OPTIONAL INFORMATION ==========");
        for optional_audio_tags in mp3_meta_data.optional_info[0..1].iter() {
            println!("Position:                                 {}", optional_audio_tags.position);
            println!("Major Version:                            {}", optional_audio_tags.major_version);
            println!("Minor Version:                            {}", optional_audio_tags.minor_version);
            if optional_audio_tags.album_movie_show.is_some(){
            println!("Album Movie Show:                         {}", optional_audio_tags.album_movie_show.to_owned().unwrap());
            }
            else{
            println!("Album Movie Show:                         {}", "Not Available".to_string());
            }
            if optional_audio_tags.bpm.is_some(){
            println!("Beats Per Minute:                         {}", optional_audio_tags.bpm.to_owned().unwrap());
            }
            else{
            println!("Beats Per Minute:                         {}", "Not Available".to_string());
            }
           if !&optional_audio_tags.composers.is_empty(){
              for composer in &optional_audio_tags.composers {
            println!("Composer:                                 {}", *composer);
              }
           }
           else{
            println!("Composer:                                 {}", "Not Available".to_string());
        }
           if !&optional_audio_tags.content_type.is_empty(){
            for genre in &optional_audio_tags.content_type {
            println!("Genre:                                    {:?}", genre);
            }
         }
         else{
            println!("Genre:                                    {}", "Not Available".to_string());
     }
         if optional_audio_tags.copyright.is_some(){
            println!("Copyright:                                {}", optional_audio_tags.copyright.to_owned().unwrap());
            }
        else{
            println!("Copyright:                                {}", "Not Available".to_string());
        }  
        if optional_audio_tags.date.is_some(){
            println!("Date:                                     {}", optional_audio_tags.date.to_owned().unwrap());
            }
            else{
            println!("Date:                                     {}", "Not Available".to_string());
            }
        if optional_audio_tags.playlist_delay.is_some(){
            println!("Playlist Delay:                           {}", optional_audio_tags.playlist_delay.to_owned().unwrap());
            }
            else{
            println!("Playlist Delay:                           {}", "Not Available".to_string());
            }            
       if optional_audio_tags.encoded_by.is_some(){
            println!("Encoded By:                               {}", optional_audio_tags.encoded_by.to_owned().unwrap());
            }
            else{
            println!("Encoded By:                               {}", "Not Available".to_string());
            }
       if !&optional_audio_tags.text_writers.is_empty(){
              for text_writers in &optional_audio_tags.text_writers {
            println!("Text Writers:                             {}", *text_writers);
              }
           }
           else{
            println!("Text Writers:                             {}", "Not Available".to_string());
        }
        if optional_audio_tags.file_type.is_some(){
            println!("File Type:                                {}", optional_audio_tags.file_type.to_owned().unwrap());
            }
            else{
            println!("File Type:                                {}", "Not Available".to_string());
            }
        if optional_audio_tags.time.is_some(){
            println!("Time:                                     {}", optional_audio_tags.time.to_owned().unwrap());
            }
            else{
            println!("Time:                                     {}", "Not Available".to_string());
            }
        if optional_audio_tags.content_group_description.is_some(){
            println!("Content Group Description:                {}", optional_audio_tags.content_group_description.to_owned().unwrap());
            }
            else{
            println!("Content Group Description:                {}", "Not Available".to_string());
            }
        if optional_audio_tags.subtitle_refinement_description.is_some(){
            println!("Subtitle Refinement Description:          {}", optional_audio_tags.subtitle_refinement_description.to_owned().unwrap());
            }
            else{
            println!("Subtitle Refinement Description:          {}", "Not Available".to_string());
            }
        if optional_audio_tags.title.is_some(){
            println!("Title:                                    {}", optional_audio_tags.title.to_owned().unwrap());
            }
            else{
            println!("Title:                                    {}", "Not Available".to_string());
            }
        if optional_audio_tags.initial_key.is_some(){
            println!("Initial Key:                              {}", optional_audio_tags.initial_key.to_owned().unwrap());
            }
            else{
            println!("Initial Key:                              {}", "Not Available".to_string());
            }
        if optional_audio_tags.language.is_some(){
            println!("Language:                                 {}", optional_audio_tags.language.to_owned().unwrap());
            }
            else{
            println!("Language:                                 {}", "Not Available".to_string());
            }
        if optional_audio_tags.length.is_some(){
            println!("Length:                                   {}", optional_audio_tags.length.to_owned().unwrap());
            }
            else{
            println!("Length:                                   {}", "Not Available".to_string());
            }
        if optional_audio_tags.media_type.is_some(){
            println!("Media Type:                               {}", optional_audio_tags.media_type.to_owned().unwrap());
            }
            else{
            println!("Media Type:                               {}", "Not Available".to_string());
            }
        if optional_audio_tags.original_album_move_show_title.is_some(){
            println!("Original Album Movie Show Title:          {}", optional_audio_tags.original_album_move_show_title.to_owned().unwrap());
            }
            else{
            println!("Original Album Movie Show Title:          {}", "Not Available".to_string());
            }
        if optional_audio_tags.original_filename.is_some(){
            println!("Original Filename:                        {}", optional_audio_tags.original_filename.to_owned().unwrap());
            }
            else{
            println!("Original Filename:                        {}", "Not Available".to_string());
            }
        if !&optional_audio_tags.original_text_writers.is_empty(){
              for original_text_writers in &optional_audio_tags.original_text_writers {
            println!("Original Text Writers:                    {}", *original_text_writers);
              }
           }
           else{
            println!("Original Text Writers:                    {}", "Not Available".to_string());
        }
        if !&optional_audio_tags.original_artists.is_empty(){
            for original_artists in &optional_audio_tags.original_artists {
            println!("Original Artists:                         {}", *original_artists);
            }
         }
         else{
            println!("Original Artists:                         {}", "Not Available".to_string());
      }
        if optional_audio_tags.original_release_year.is_some(){
            println!("Original Release Year:                    {}", optional_audio_tags.original_release_year.to_owned().unwrap());
        }
        else{
            println!("Original Release Year:                    {}", "Not Available".to_string());
        }
        if optional_audio_tags.file_owner.is_some(){
            println!("File Owner:                               {}", optional_audio_tags.file_owner.to_owned().unwrap());
        }
        else{
            println!("File Owner:                               {}", "Not Available".to_string());
        }
        if !&optional_audio_tags.performers.is_empty(){
            for performers in &optional_audio_tags.performers {
            println!("Performers:                               {}", *performers);
            }
         }
         else{
            println!("Performers:                               {}", "Not Available".to_string());
        }
        if optional_audio_tags.band.is_some(){
            println!("Band:                                     {}", optional_audio_tags.band.to_owned().unwrap());
        }
        else{
            println!("Band:                                     {}", "Not Available".to_string());  
        }
        if optional_audio_tags.conductor.is_some(){
            println!("Conductor:                                {}", optional_audio_tags.conductor.to_owned().unwrap());
        }
        else{
            println!("Conductor:                                {}", "Not Available".to_string());  
        }
        if optional_audio_tags.interpreted.is_some(){
            println!("Interpreted:                              {}", optional_audio_tags.interpreted.to_owned().unwrap());
        }
        else{
            println!("Interpreted:                              {}", "Not Available".to_string());  
        }
        if optional_audio_tags.part_of_a_set.is_some(){
            println!("Part Of A Set:                            {}", optional_audio_tags.part_of_a_set.to_owned().unwrap());
        }
        else{
            println!("Part Of A Set:                            {}", "Not Available".to_string());  
        }
        if optional_audio_tags.publisher.is_some(){
            println!("Publisher:                                {}", optional_audio_tags.publisher.to_owned().unwrap());
        }
        else{
            println!("Publisher:                                {}", "Not Available".to_string());  
        }
        if optional_audio_tags.track_number.is_some(){
            println!("Track Number:                             {}", optional_audio_tags.track_number.to_owned().unwrap());
        }
        else{
            println!("Track Number:                             {}", "Not Available".to_string());  
        }
        if optional_audio_tags.recording_dates.is_some(){
            println!("Recording Dates:                          {}", optional_audio_tags.recording_dates.to_owned().unwrap());
        }
        else{
            println!("Recording Dates:                          {}", "Not Available".to_string());  
        }
        if optional_audio_tags.internet_radio_station_name.is_some(){
            println!("Internet Radio Station Name:              {}", optional_audio_tags.internet_radio_station_name.to_owned().unwrap());
        }
        else{
            println!("Internet Radio Station Name:              {}", "Not Available".to_string());  
        }
        if optional_audio_tags.internet_radio_station_owner.is_some(){
            println!("Internet Radio Station Owner:             {}", optional_audio_tags.internet_radio_station_owner.to_owned().unwrap());
        }
        else{
            println!("Internet Radio Station Owner:             {}", "Not Available".to_string());  
        }
        if optional_audio_tags.size.is_some(){
            println!("Size:                                     {}", optional_audio_tags.size.to_owned().unwrap());
        }
        else{
            println!("Size:                                     {}", "Not Available".to_string());  
        }
        if optional_audio_tags.international_standard_recording_code.is_some(){
            println!("International Standard Recording Code:    {}", optional_audio_tags.international_standard_recording_code.to_owned().unwrap());
        }
        else{
            println!("International Standard Recording Code:    {}", "Not Available".to_string());  
        }
        if optional_audio_tags.soft_hard_setting.is_some(){
            println!("Soft Hard Setting:                        {}", optional_audio_tags.soft_hard_setting.to_owned().unwrap());
        }
        else{
            println!("Soft Hard Setting:                        {}", "Not Available".to_string());  
        }
        if optional_audio_tags.year.is_some(){
            println!("Year:                                     {}", optional_audio_tags.year.to_owned().unwrap());
        }
        else{
            println!("Year:                                     {}", "Not Available".to_string());  
        }
        if optional_audio_tags.involved_people.is_some(){
            println!("Involved People:                          {}", optional_audio_tags.involved_people.to_owned().unwrap());
        }
        else{
            println!("Involved People:                          {}", "Not Available".to_string());  
        }
        if !&optional_audio_tags.commercial_info_url.is_empty(){
            for commercial_info_url in &optional_audio_tags.commercial_info_url {
            println!("Commercial Info URL:                      {:?}", *commercial_info_url);
            }
         }
         else{
            println!("Commercial Info URL:                      {}", "Not Available".to_string());
        }
        if optional_audio_tags.copyright_info_url.is_some(){
            println!("Copyright_Info URL:                       {:?}", &optional_audio_tags.copyright_info_url.as_ref());
        }
        else{
            println!("Copyright Info URL:                       {}", "Not Available".to_string());  
        }
        if optional_audio_tags.official_webpage.is_some(){
            println!("Official Webpage:                         {:?}", &optional_audio_tags.official_webpage.as_ref());
        }
        else{
            println!("Official Webpage:                         {}", "Not Available".to_string());  
        }
        if !&optional_audio_tags.official_artist_webpage.is_empty(){
            for official_artist_webpage in &optional_audio_tags.official_artist_webpage {
            println!("Official Artist Webpage:                  {:?}", *official_artist_webpage);
            }
         }
         else{
            println!("Official Artist Webpage:                  {}", "Not Available".to_string());
        }
        if optional_audio_tags.official_audio_source_webpage.is_some(){
            println!("Official Audio Source Webpage:            {:?}", &optional_audio_tags.official_audio_source_webpage.as_ref());
        }
        else{
            println!("Official Audio Source Webpage:            {}", "Not Available".to_string());  
        }
        if optional_audio_tags.official_internet_radio_webpage.is_some(){
            println!("Official Internet Radio Webpage:          {:?}", &optional_audio_tags.official_internet_radio_webpage.as_ref());
        }
        else{
            println!("Official Internet Radio Webpage:          {}", "Not Available".to_string());  
        }
        if optional_audio_tags.payment_url.is_some(){
            println!("Payment URL:                              {:?}", &optional_audio_tags.payment_url.as_ref());
        }
        else{
            println!("Payment URL:                              {}", "Not Available".to_string());  
        }
        if optional_audio_tags.publishers_official_webpage.is_some(){
            println!("Publishers Official Webpage:              {:?}", &optional_audio_tags.publishers_official_webpage.as_ref());
        }
        else{
            println!("Publishers Official Webpage:              {}", "Not Available".to_string());  
        }
    
    
      } // End of For
    
    } // End of if
    else
    {
        println!("\n\nOptional Informaion is Not Available.")
    }
    
}

fn main() {
    let file = match env::args().skip(1).next() {
        Some(argument) => argument,
        None => {
            println!("\nPlease provide a mp3 filename.");
            return
        }
    };
    let mp3_meta_data = mp3_metadata::read_from_file(&file).expect("File error");
    
    
    println!("\nShowing MP3 MetaData for file : {}", &file);
    println!("______________________________________________________");

    print_tags(&mp3_meta_data);
    print_first_frame(&mp3_meta_data);
    print_optional_information(&mp3_meta_data);
   
    println!("\n========== END ==========");
     
}