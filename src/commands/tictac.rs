use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;
use std::vec;

#[command]
pub async fn tictac(ctx: &Context, msg: &Message,) -> CommandResult {
    let mut positions = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]; //the board is actually a 3x3 matrix (or an vector made of 3 more vectors) :')
    let mut x_turn = true; //true = X, false = O
    let mut choose_r: i32 = 0;
    let mut choose_c: i32 = 0;
    let mut passone = true;
    let mut passtwou = true;
    // let player_twou  = args.single::<String>().unwrap();
    msg.channel_id
        .say(&ctx.http, positionstostring(&positions))
        .await?;
    msg.channel_id.say(&ctx.http, "pelase input a row").await?;
    'outer: loop {
        loop {
            loop {
                if let Some(answer) = &msg
                    .author
                    .await_reply(&ctx)
                    .timeout(Duration::from_secs(30))
                    .await
                {
                    if answer.content[..1] == String::from("x")
                        || answer.content[..1] == String::from("X")
                    {
                        break 'outer;
                    }
                    if !answer.content[..1].chars().all(char::is_numeric) {
                        let _ = answer.reply(ctx, "please input a number (X to quit)").await;
                        passone = false;
                    } else if answer.content[..1].parse::<i32>().unwrap() <= 3
                        && answer.content[..1].parse::<i32>().unwrap() >= 1
                    {
                        choose_r = answer.content[..1].parse::<i32>().unwrap() as i32 - 1;
                    } else {
                        let _ = answer
                            .reply(ctx, "please type a valid number (X to quit)")
                            .await;
                        passone = false;
                    }
                } else {
                    let _ = msg.reply(ctx, "No answer within 10 seconds.").await;
                    break 'outer;
                };
                if passone == true {
                    break;
                }
            }
            //imputing the column
            msg.channel_id
                .say(&ctx.http, "pelase input a column")
                .await?;
            loop {
                let mut passone = true;
                if let Some(answer) = &msg
                    .author
                    .await_reply(&ctx)
                    .timeout(Duration::from_secs(30))
                    .await
                {
                    if answer.content[..1] == String::from("x")
                        || answer.content[..1] == String::from("X")
                    {
                        break 'outer;
                    }
                    if !answer.content[..1].chars().all(char::is_numeric) {
                        let _ = answer.reply(ctx, "please input a number (X to quit)").await;
                        passone = false;
                    } else if answer.content[..1].parse::<i32>().unwrap() <= 3
                        && answer.content[..1].parse::<i32>().unwrap() >= 1
                    {
                        choose_c = answer.content[..1].parse::<i32>().unwrap() as i32 - 1;
                    } else {
                        let _ = answer
                            .reply(ctx, "please type a valid number (X to quit)")
                            .await;
                        passone = false;
                    }
                } else {
                    let _ = msg.reply(ctx, "No answer within 10 seconds.").await;
                    break 'outer;
                };
                if passone == true {
                    break;
                }
            }
            if positions[choose_r as usize][choose_c as usize] != 0 {
                //here we check if the spot is taken, if it is we will restart the loop
                msg.channel_id
                    .say(&ctx.http, "this spot is taken, please choose another one")
                    .await?;
                passtwou = false;
            }
            if passtwou == true {
                break;
            }
        }
        if x_turn == true {
            positions = mark(&positions, choose_r, choose_c, 'x'); //marks the spot with an x if it is X's turn
        } else {
            positions = mark(&positions, choose_r, choose_c, 'o'); // marks the spot with an o if it is O's turn
        }
        msg.channel_id
            .say(&ctx.http, positionstostring(&positions))
            .await?;
        x_turn = !x_turn;
        let wincheck = checkwin(&positions); //checks if someone has won an saves to wincheck var
        if wincheck == 1 {
            //if its 1, X wins, we print the board and exit the loop
            msg.channel_id.say(&ctx.http, "X wins!").await?;
            break;
        } else if wincheck == 2 {
            //if its -2, O wins, we print the board and exit the loop
            msg.channel_id.say(&ctx.http, "O wins!").await?;
        } else if wincheck == 0 {
            //if its 0, nobody has won, and we dont break out of the loop
            continue;
        }
    }
    Ok(())
}

fn positionstostring(positions: &Vec<Vec<i32>>) -> String {
    //converts the board (matrix/vectors) to a string using emojis to look cool :P
    let mut string = String::new();
    for i in 0..3 {
        string.push_str("\n");
        for j in 0..3 {
            string.push_str(&positions[i][j].to_string());
        }
    }
    let result = string
        .replace("0", "⬜")
        .replace("-1", "⭕")
        .replace("1", "❌"); //replaces the -1(O marks) and 1(X marks) with the emojis
    result
}
fn mark(positions: &Vec<Vec<i32>>, x: i32, y: i32, turn: char) -> Vec<Vec<i32>> {
    //receives the board, the row and column, and the turn (X or O) and marks the spot
    let mut newpositions = positions.clone();
    if turn == 'x' {
        newpositions[(x) as usize][(y) as usize] = 1;
    } else {
        newpositions[(x) as usize][(y) as usize] = -1;
    }
    newpositions
}
fn checkwin(positions: &Vec<Vec<i32>>) -> u32 {
    //checks if the sum of roll/column/diagonal is 3(X wins return 1) or -3(O wins return 2), if none, return 0
    let sumfirst = positions[0][0] + positions[1][0] + positions[2][0];
    if sumfirst == 3 {
        return 1;
    } else if sumfirst == -3 {
        return 2;
    }
    let sumsecond = positions[0][1] + positions[1][1] + positions[2][1];
    if sumsecond == 3 {
        return 1;
    } else if sumsecond == -3 {
        return 2;
    }
    let sumthird = positions[0][2] + positions[1][2] + positions[2][2];
    if sumthird == 3 {
        return 1;
    } else if sumthird == -3 {
        return 2;
    }
    let sumfirstdiagonal = positions[0][0] + positions[1][1] + positions[2][2];
    if sumfirstdiagonal == 3 {
        return 1;
    } else if sumfirstdiagonal == -3 {
        return 2;
    }
    let sumseconddiagonal = positions[0][2] + positions[1][1] + positions[2][0];
    if sumseconddiagonal == 3 {
        return 1;
    } else if sumseconddiagonal == -3 {
        return 2;
    } else {
        return 0;
    }
}
