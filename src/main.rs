use pulsectl::controllers::{DeviceControl, AppControl};
use pulsectl::controllers::SinkController;
use std::io;
use log::*;
use std::ops::Index;
use clap::{App, Arg};
use pulsectl::ControllerError;


fn current_sink(handler: &mut SinkController, long_format: bool)
{
    let mut to_print = "N/A".to_string();

    if let Ok(default_device) = handler.get_default_device()
    {
        if let Some(description) = default_device.description
        {
            if long_format
            {
                to_print = description;
            }

            else
            {
                if let Some(l) = description.split_whitespace().next()
                {
                    to_print = l.to_string()
                }
            }
        }
    };

    println!("{}", to_print);
}

fn cycle_sinks(handler: &mut SinkController) -> Result<(), ()>
{
    let devices = match handler.list_devices()
    {
        Ok(devices) => { devices },
        Err(e) => { return Err(()); }
    };

    if devices.len() == 1
    {
        return Ok(());
    }

    //handler.set_default_device()
    let default_device = handler.get_default_device().unwrap();
    let current_index  = default_device.index as usize;
    let mut new_index = 0_usize;

    if current_index + 1 < devices.len()
    {
        // new_index is initialized as 0, so "else" statement is needed.
        new_index = current_index + 1;
    }

    println!("new_index: {}", new_index);


    match &devices[new_index].clone().name
    {
        None => { return Err(()); }
        Some(name) =>
        {
            handler.set_default_device(name).ok();
        }
    }

    match handler.list_applications()
    {
        Ok(apps) =>
        {
            for app in apps
            {
                handler.move_app_by_index(app.index, new_index as u32).ok();
            }
        }

        Err(e) =>
        {
            return Err(());
        }
    }

    Ok(())
}

fn main()
{
    let mut handler = SinkController::create().unwrap();

    let matches = App::new("MyApp")
        .arg(Arg::with_name("cycle").short("c").conflicts_with("current_sink"))
        .arg(Arg::with_name("current_sink").short("s"))
        .arg(Arg::with_name("current_sink_long").short("l"))
        .get_matches();

    if matches.is_present("current_sink_long")
    {
        current_sink(&mut handler, true)
    }

    else if matches.is_present("current_sink")
    {
        current_sink(&mut handler, true)
    }

    if matches.is_present("cycle")
    {
        println!("Cycling sinks...");
        cycle_sinks(&mut handler).ok();
    }

    /*println!("Playback Devices: ");
    for device in &devices
    {
        println!(
            "[Index: {}] {}, [Volume: {}]",
            device.index,
            device.description.as_ref().unwrap(),
            device.volume
        );
    }*/


    //let mut input = String::new();

    /*info!("Select an index: ");
    //io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");*/

    //println!("Increasing volume by 5%...");

    //let device_index = input.trim().parse().expect("Invalid number");
    //handler.increase_device_volume_by_percent(device_index, 0.05);


    //let device_index = input.trim().parse().expect("Invalid number");

    /*println!(
        "Volume set to [Volume: {}]",
        handler
            .get_device_by_index(device_index)
            .expect("Failed to get device by index")
            .volume
    )*/
}
