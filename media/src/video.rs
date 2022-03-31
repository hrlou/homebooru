pub mod prelude {
    pub use crate::imager::{self, prelude::*};
    pub use crate::video::*;

    pub extern crate ffmpeg_next as ffmpeg;
    pub use ffmpeg::{
        decoder,
        format::{input, Pixel, context::Input, stream::Stream},
        frame::Video as Frame,
        media::Type as MediaType,
        software::scaling::{
            flag::Flags as ContextFlags,
            context::Context,
        },
        codec::{
            decoder::Video as VideoDecoder,
            context::Context as CodecContext,
        },
    };
}

pub mod frame {
    use crate::video::prelude::*;

    pub fn parse(frame: &Frame) -> Option<Vec<u8>> {
        use std::io::Write;

        let data = frame.data(0);
        let stride = frame.stride(0);
        let chunk_size: usize = 3;
        let byte_width: usize = chunk_size * frame.width() as usize;
        let height: usize = frame.height() as usize;
    
        let mut buf: Vec<u8> = vec![];
        buf.reserve((frame.width() * frame.height() * chunk_size as u32) as usize);
        for line in 0..height {
            let begin = line * stride;
            let end = begin + byte_width;
            match buf.write_all(&data[begin..end]) {
                Ok(_) => {},
                Err(_) => return None,
            }
        }
        Some(buf)
    }
    
    pub fn image(frame: &Frame) -> Option<RgbBuffer> {    
        match parse(frame) {
            Some(buf) => RgbBuffer::from_raw(frame.width(), frame.height(), buf),
            None => None,
        }   
    }
}

use prelude::*;
use std::{path::Path};

// public exposure
pub use crate::imager::RgbBuffer;
pub use ffmpeg::init;

#[derive(Default)] 
pub struct Video {
    pub input: Option<Input>,
    // pub stream: Option<Stream<'static>>,
    pub index: Option<usize>,
}

impl Video {
    pub fn new<T>(path: T) -> Option<Video>
    where
        T: AsRef<Path>,
    {
        let i = input(&path);
        match i {
            Ok(i) => {
                let mut stream = Video::default();
                stream.input = Some(i);
                Some(stream)
            },
            Err(_) => None,
        }
    }

    fn input(&self) -> Result<&Input, ffmpeg::Error> {
        match self.input.as_ref() {
            Some(input) => Ok(input),
            None => Err(ffmpeg::Error::InvalidData),
        }
    }

    fn index(&self) -> Result<usize, ffmpeg::Error> {
        match self.index {
            Some(index) => Ok(index),
            None => return Err(ffmpeg::Error::StreamNotFound),        }
    }

    fn stream(&self) -> Result<Stream, ffmpeg::Error> {
        match self.input()?.stream(self.index()?) {
            Some(stream) => Ok(stream),
            None => return Err(ffmpeg::Error::StreamNotFound),
        }
    }

    pub fn setup_stream<'a>(&mut self, kind: Option<MediaType>) -> Result<(), ffmpeg::Error> {
        let kind = kind.unwrap_or(MediaType::Video);
        let stream = self.input.as_ref().unwrap().streams().find(|stream| {
            let codec = CodecContext::from_parameters(stream.parameters()).unwrap();
            codec.medium() == kind
        }).ok_or(ffmpeg::Error::StreamNotFound)?;
        self.index = Some(stream.index());
        Ok(())
    }

    pub fn video(&self) -> Result<(VideoDecoder, Context), ffmpeg::Error> {
        let stream = self.stream()?;
        let context = CodecContext::from_parameters(stream.parameters())?;
        let video = context.decoder().video()?;
        let scaler = Context::get(
            video.format(),
            video.width(),
            video.height(),
            Pixel::RGB24,
            video.width(),
            video.height(),
            ContextFlags::BILINEAR,
        )?;
        Ok((video, scaler))
    }

    pub fn process_frames<F>(&mut self, mut process_frame: F) -> Result<(), ffmpeg::Error> 
    where 
        F: FnMut(&Frame, usize) -> bool,
    {
        let (mut video, mut scaler) = self.video()?;
        let mut frame_index = 0;

        let mut receive_frames =
            |decoder: &mut decoder::Video| -> Result<bool, ffmpeg::Error> {
                let mut decoded = Frame::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Frame::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    let r = process_frame(&rgb_frame, frame_index);
                    frame_index += 1;
                    if !r {
                        return Ok(false);
                    }
                }
                Ok(true)
            };
    
        let input: &mut Input = self.input.as_mut().unwrap();
        for (s, packet) in input.packets() {
            if s.index() == self.index.unwrap() {
                video.send_packet(&packet)?;
                if receive_frames(&mut video)? == false {
                    break;
                }
            }
        }
        video.send_eof()?;
        receive_frames(&mut video)?;
    
        Ok(())
    }
}