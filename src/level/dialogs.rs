use std::collections::HashMap;

use crate::dialog::{Branching, Dialog, DialogEffect, DialogId, DialogStick, Replica, Variant};

//   start
//   *
//   |
//   ^
//   |
//   *
//   |
//  ^ ^
//  | |
//  * *
//  |
//  ^
//  |
//  *

pub const SLEEPING_FORMIDABLE_DOG_DIALOG: usize = 1;

pub const END_DIALOG_FORMIDABLE_DOG_JOINED: usize = 1;

pub fn sleeping_formidable_dog_dialog() -> Dialog {
    let (root_id, sticks) = sleeping_formidable_dog();
    return Dialog::from(
        DialogId(COURIER_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn sleeping_formidable_dog() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("[Спит] хррр... хррр... хрр...".to_string()),
        ]
    );

    let mut after_wake_up_stick = DialogStick::from(1);
    after_wake_up_stick.replicas.extend(
        vec![
            Replica::from_text("Ыыыууу... Что... [зевает] такое, Грозьный?".to_string()),
        ]
    );

    let mut go_with_me_stick = DialogStick::from(2);

    go_with_me_stick.replicas.extend(
        vec![
            Replica::from_text("Что еще за повестька?".to_string()),
        ]
    );

    let mut joined_stick = DialogStick::from(3);

    joined_stick.replicas.extend(
        vec![
            Replica::from_text("Лядня, пойдем разберемся.".to_string()),
        ]
    );

    go_with_me_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create_with_effect(
                    "Ыыыу, подозреваю, что делё опять в доме.".to_string(),
                    joined_stick.id,
                    DialogEffect::EndDialog(Some(END_DIALOG_FORMIDABLE_DOG_JOINED)),
                ),
            ],
        }
    );

    let mut you_are_rude_stick = DialogStick::from(4);

    you_are_rude_stick.replicas.extend(
        vec![
            Replica::from_text("Ыыыыууу! Ты чего такой грубий?! Не пойду ни кудя за етё!".to_string()),
        ]
    );

    after_wake_up_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create_with_effect(
                    "Пойдем сё мной. Мне дали повестьку в судь.".to_string(),
                    go_with_me_stick.id,
                    DialogEffect::ReplaceDialog,
                ),
                Variant::create_with_effect(
                    "Всьтавай давай, шавка парщивая! Сколько можно валяться? Мне повестьку в суть дали!".to_string(),
                    you_are_rude_stick.id,
                    DialogEffect::EndDialog(None),
                ),
            ],
        }
    );

    main_stick.replicas.extend(
        vec![
            Replica::from_text("[Спит] хррр... хррр... хрр...".to_string()),
        ]
    );

    main_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create_with_effect(
                    "Подъемь!".to_string(),
                    after_wake_up_stick.id,
                    DialogEffect::ReplaceDialog,
                ),
            ],
        }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(after_wake_up_stick.id, after_wake_up_stick);
    pool.insert(go_with_me_stick.id, go_with_me_stick);
    pool.insert(joined_stick.id, joined_stick);
    pool.insert(you_are_rude_stick.id, you_are_rude_stick);

    return (root_id, pool);
}

//         start
//         *
//         |
//         *
//         |
//         *
//  -->^ ^ ^ ^ ^
//  |  | | | | |
//  |  * * * * *->end
//  |  | | | |
//  ---* * * *->end
//  |    | |--
//  ----^ ^  |
//  |     |  *
//  |     *  |
//  |    ^ ^ |---
//  |    | |    |
//  |--- * *    |
//  |      |    |
//  |      end  |
//  |<----------|
//

pub const COURIER_DIALOG: usize = 2;

pub const END_DIALOG_NECK_TWISTED: usize = 1;
pub const END_DIALOG_AGENDA_TAKEN: usize = 2;

pub fn courier_dialog() -> Dialog {
    let (root_id, sticks) = courier_dialog_body();
    return Dialog::from(
        DialogId(COURIER_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn courier_dialog_body() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Здравстуйте! У меня к вам очень важное дело.".to_string()),
            Replica::from_text("Вам повестка в суд! Вы должны расписаться здесь и здесь, а еще вот тут. Пожалуйста, вот вам ручка.".to_string()),
            Replica::from_text("Только не забудьте её мне вернуть, хорошо?".to_string()),
        ]
    );

    let mut it_is_mistake_question_stick = DialogStick::from(1);
    it_is_mistake_question_stick.replicas.push(
        Replica::from_text("Ну, тут Ваш адрес, Ваше имя. Грозный Личик, так ведь?".to_string())
    );
    it_is_mistake_question_stick.replicas.push(
        Replica::from_text("Так что определенно ваша повестка. Подпишите?".to_string())
    );

    let mut why_question_stick = DialogStick::from(2);
    why_question_stick.replicas.push(
        Replica::from_text("Я просто курьер, особых подробностей не знаю.".to_string())
    );
    why_question_stick.replicas.push(
        Replica::from_text("Но вроде как вы объвиняетесь в незаконном владении имуществом, \
        а именно Вашим, или, если обвинения не беспочвенны, не Вашем домом.".to_string())
    );

    let mut dont_say_any = DialogStick::from(21);
    dont_say_any.replicas.push(
        Replica::from_text("Ну так что? Подписывать будете?".to_string())
    );
    let mut furiously_twist_neck = DialogStick::from(22);
    furiously_twist_neck.replicas.push(
        Replica::from_text("Что вы... Грозный, ввваааааа! Эу...".to_string())
    );
    why_question_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create_with_effect("[Промолчать]".to_string(), dont_say_any.id, DialogEffect::ReplaceDialog),
                Variant::create_with_effect("Ыыыыыыууууу!!!! \
                Этё мой дём!!!!! [Свернуть шею]".to_string(),
                                            furiously_twist_neck.id,
                                            DialogEffect::EndDialog(Some(END_DIALOG_NECK_TWISTED)),
                ),
            ],
        }
    );

    let mut when_in_court = DialogStick::from(3);
    when_in_court.replicas.push(
        Replica::from_text("В повесте будет указано. Обычно мы уведомляем миними за неделю.".to_string())
    );
    when_in_court.replicas.push(
        Replica::from_text("У Вас же впереди целых две. В случае неявки вам будут назначен штраф и повторная повестка.".to_string())
    );
    when_in_court.replicas.push(
        Replica::from_text("Если вы не явитесь вообще... Ну, наверное, этим уже будет заниматься милиция.".to_string())
    );

    let mut ok = DialogStick::from(4);
    ok.replicas.push(
        Replica::from_text("Да, да, вот тут... Ага и в этих местах... Спасибо! Всего доброго!".to_string())
    );

    let mut twist_neck = DialogStick::from(5);
    twist_neck.replicas.push(
        Replica::from_text("Да что вы...! Эуу....!".to_string())
    );
    main_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create("Ыыыу, ето ошибка! Мне не должно ничего приходить!".to_string(), it_is_mistake_question_stick.id),
                Variant::create("Почему мне присля этя повесьтька?".to_string(), why_question_stick.id),
                Variant::create("Когдя мне надо в суд?".to_string(), when_in_court.id),
                Variant::create_with_effect(
                    "Хм. Давайте я расписюсь.".to_string(),
                    ok.id,
                    DialogEffect::EndDialog(Some(crate::level::END_DIALOG_AGENDA_TAKEN)),
                ),
                Variant::create_with_effect(
                    "[Свернуть шею]".to_string(),
                    twist_neck.id,
                    DialogEffect::EndDialog(Some(crate::level::END_DIALOG_NECK_TWISTED)),
                ),
            ],
        }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(dont_say_any.id, dont_say_any);
    pool.insert(furiously_twist_neck.id, furiously_twist_neck);
    pool.insert(it_is_mistake_question_stick.id, it_is_mistake_question_stick);
    pool.insert(why_question_stick.id, why_question_stick);
    pool.insert(when_in_court.id, when_in_court);
    pool.insert(ok.id, ok);
    pool.insert(twist_neck.id, twist_neck);

    return (root_id, pool);
}