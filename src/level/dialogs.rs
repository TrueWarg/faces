use std::collections::HashMap;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Component, KeyCode, NextState, Query, Res, ResMut, Transform};
use crate::core::states::GameState;

use crate::dialog::{Branching, Dialog, DialogEffect, DialogId, DialogStick, Replica, Variant};
use crate::interaction::interactors::{ActiveInteractor, detect_active_interaction, PassiveInteractor};

pub trait HasDialogId {
    fn dialog_id(&self) -> usize;
}

pub fn dialog_starts<T: HasDialogId + Component>(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform, &T)>,
    mut dialog_id_query: Query<(&mut DialogId)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if !(keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE)) {
        return;
    }
    for (interactor, transform, has_dialog) in interactors.iter() {
        let is_interacting = detect_active_interaction(&active, (interactor, transform));
        if is_interacting {
            match dialog_id_query.get_single_mut() {
                Ok(mut dialog_id) => dialog_id.0 = has_dialog.dialog_id(),
                Err(_) => {
                    commands.spawn(DialogId(has_dialog.dialog_id()));
                }
            }
            next_game_state.set(GameState::Dialog);
        }
    }
}

//  START
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

//       START
//         *
//         |
//         *
//         |
//         *
//  -->^ ^ ^ ^ ^
//  |  | | | | |
//  |  * * * * * -> END
//  |  | | | |
//  ---* * * * -> END
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
                    DialogEffect::EndDialog(Some(END_DIALOG_AGENDA_TAKEN)),
                ),
                Variant::create_with_effect(
                    "[Свернуть шею]".to_string(),
                    twist_neck.id,
                    DialogEffect::EndDialog(Some(END_DIALOG_NECK_TWISTED)),
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

//   START
//     *
//     |
//     *
//     |
//     *
//    | |
//    ^ ^
//    | |
//    * * -> END
//  | | |
//  ^ ^ ^
//  | | |
//  * * * -> END
//  ---
//  | |
//  ^ ^
//  | |
//  * * -> END
pub const DREVNIRA_DIALOG: usize = 3;

pub const END_DIALOG_DREVNIRA_BEATEN: usize = 1;

pub fn old_woman_drevnira_dialog() -> Dialog {
    let (root_id, sticks) = old_woman_drevnira();
    return Dialog::from(
        DialogId(DREVNIRA_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn old_woman_drevnira() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("[Перед тобой стоит высокая и тощая старушка с малеькой головой и кланится в какое-то закрытое окно. При этом она говорит...] ".to_string()),
            Replica::from_text("Перо, дай мне перо, Пьерро. Пьерро, Пьерро, дай мне перо, перо....".to_string()),
            Replica::from_text("Дай мне перо, Пьерро, дай мне перо, перо, Пьерро, дай мне перо...".to_string()),
        ]
    );

    let mut again_talking = DialogStick::from(1);
    again_talking.replicas.extend(
        vec![
            Replica::from_text("[Старушка не обращает на тебя внимание и продолжает...] Да Пьерро, перо, дай мне перо!..".to_string()),
        ]
    );

    let mut skip = DialogStick::from(2);
    skip.replicas.extend(
        vec![
            Replica::from_text("Ну Пьерро, дай мне перо...".to_string()),
        ]
    );

    let mut i_have = DialogStick::from(3);
    i_have.replicas.extend(
        vec![
            Replica::from_text("Перро, дай мне перо, Пьерро, дай мне перо...".to_string()),
        ]
    );

    let mut beaten = DialogStick::from(4);
    beaten.replicas.extend(
        vec![
            Replica::from_text("[Мерзкая старуха ойкнула завалилась на бок. \
            Ты ее вырубил и теперь она долго будет спать.]".to_string()),
        ]
    );

    i_have.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "[Долбануть бабку головой об окно...]".to_string(),
                beaten.id,
                DialogEffect::EndDialog(Some(END_DIALOG_DREVNIRA_BEATEN)),
            ),
            Variant::create_with_effect(
                "[Молча уйти]".to_string(),
                skip.id,
                DialogEffect::EndDialog(None),
            ),
        ],
    }
    );

    let mut go_away = DialogStick::from(5);
    go_away.replicas.extend(
        vec![
            Replica::from_text("[Старушка резко поворачивается к тебе и столь же \
            резко вопит мерзким голосом:] Иди ты!!!\n[Затем отворачивается к окну и \
            продолжает...] Перо, мне нужно перо, Пьерро, дай мне перо...".to_string()),
        ]
    );

    go_away.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "ЫЫЫЫУУУУ!!!! [Долбануть бабку головой об окно...]".to_string(),
                beaten.id,
                DialogEffect::EndDialog(Some(END_DIALOG_DREVNIRA_BEATEN)),
            ),
            Variant::create_with_effect(
                "Воть и пойду!".to_string(),
                skip.id,
                DialogEffect::EndDialog(None),
            ),
        ],
    }
    );

    again_talking.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "У меня есть для тебя перо!".to_string(),
                i_have.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "[Потрогать по плечу]".to_string(),
                go_away.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Ай, пойду я.".to_string(),
                skip.id,
                DialogEffect::EndDialog(None),
            ),
        ],
    }
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Эй, ти!".to_string(),
                again_talking.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "[Молча уйти]".to_string(),
                skip.id,
                DialogEffect::EndDialog(None),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(again_talking.id, again_talking);
    pool.insert(skip.id, skip);
    pool.insert(beaten.id, beaten);
    pool.insert(i_have.id, i_have);
    pool.insert(go_away.id, go_away);

    return (root_id, pool);
}

//      START
//        *
//        |
//        *
//        |
//        *
//    | | | | |
//    ^ ^ ^ ^ ^
//    | | | | |
//  |>* * * * * -> END
//  |_|_|_| |
//         END

pub const BLOND_FIRST_DIALOG: usize = 4;

pub const END_DIALOG_BLOND_FIRST_ACCEPTED: usize = 1;
pub const END_DIALOG_BLOND_FIRST_DENIED: usize = 2;

pub fn blond_first_dialog() -> Dialog {
    let (root_id, sticks) = blond_first();
    return Dialog::from(
        DialogId(BLOND_FIRST_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn blond_first() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("[Ты видишь человека, озабоченно поглядывающего по сторонам. Он одет в разодранный\n\
            пиджак, на его левом ботинке развязаны шнурки. У человека светлые волосы, легкая щетина на щеказ. Так же ты замечаешь\n\
            что его верхняя губа приподнята, и мерзко подрагивает. Увидев тебя он закричал...] ".to_string()),
            Replica::from_text("Сударъ, сударъ! Пожалуйста, судар, вылушайте меня! Мне нужна помощь! Прошу вас!".to_string()),
            Replica::from_text("Сударъ, на меня напали! Меня избили и забрали мои две пачки пельменей!\n\
            Пожалуйста, помогите мне! Я вижу, что вы грозный боец и сможете вернуть мне ее у избивных меня негодяев.\
            Это не за даром, я отдам вам часть пельмений!".to_string()),
        ]
    );

    let mut who = DialogStick::from(1);
    who.replicas.extend(
        vec![
            Replica::from_text("Четыре мерзких гопника! Они напали внезано, повалили меня и отпинали!".to_string()),
        ]
    );

    let mut wheree = DialogStick::from(2);
    wheree.replicas.extend(
        vec![
            Replica::from_text("На сереро-востоке! Рядом со скамейками в парке! Может даже, что эти уюлюдки все еще там!\n\
            Уже варят наверное мое пельмени!".to_string()),
        ]
    );

    let mut when = DialogStick::from(3);
    when.replicas.extend(
        vec![
            Replica::from_text("Примерно час назад... Ох, главное, чтобы они не сварили мои пельмени! \
            Хотя, судя по их рожам, они их могут съесть сырыми...".to_string()),
        ]
    );

    let mut ok = DialogStick::from(4);
    ok.replicas.extend(
        vec![
            Replica::from_text("Ух, сударъ, благордарю вас! Удачи вам! Ну же, идите на северо-восток к скамейкам!".to_string()),
        ]
    );

    let mut deny = DialogStick::from(5);
    deny.replicas.extend(
        vec![
            Replica::from_text("Ну сударъ... Эх.".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create(
                "Ктё тебя изьбиль и сколько их былё?".to_string(),
                who.id,
            ),
            Variant::create(
                "Где тебя изьбили?".to_string(),
                wheree.id,
            ),
            Variant::create(
                "Когдя это произошлё?".to_string(),
                when.id,
            ),
            Variant::create_with_effect(
                "Лядня, я принесю тебе пельмени.".to_string(),
                ok.id,
                DialogEffect::EndDialog(Some(END_DIALOG_BLOND_FIRST_ACCEPTED)),
            ),
            Variant::create_with_effect(
                "У меня неть времени.".to_string(),
                deny.id,
                DialogEffect::EndDialog(Some(END_DIALOG_BLOND_FIRST_DENIED)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(who.id, who);
    pool.insert(wheree.id, wheree);
    pool.insert(when.id, when);
    pool.insert(ok.id, ok);
    pool.insert(deny.id, deny);

    return (root_id, pool);
}

//      START
//        *
//        |
//        *
//        |
//        *
//       | |
//       ^ ^
//       | |
//       * * -> END
//      | |
//      ^ ^ -> END
//      |
//      *
//      |
//      *
//      |
//      ^
//      |
//      *
//     | |
//     ^ ^
//     | |
//     * * -> END
//     |
//    END
pub const GOPNIKS_DIALOG: usize = 5;

pub const END_DIALOG_GOPNIKS_DIALOG_FIGHT: usize = 1;
pub const END_DIALOG_GOPNIKS_DIALOG_ASK_BLOND: usize = 2;

pub fn gopniks_dialog() -> Dialog {
    let (root_id, sticks) = gopniks();
    return Dialog::from(
        DialogId(GOPNIKS_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn gopniks() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("[Перед тобой четверо... индивидумов. Один из них сидит на коточках, у него хмурое лицо и сморщеный лоб.\n\
            Слева от него стоит громадина в красных лосинах. Поодаль ты видишь двух близнецов с глупым выраженим на лицах.\n\
            Тот, что сидит на корточках, судя по всему, главарь, зло смотрит на тебя из подлобья.]".to_string()),
            Replica::from_text("Херли тебе надо, а?".to_string()),
            Replica::from_text("[Гигантсвий гопник в красных лосинах заржал и добавил:] \
            Гы! Люлей давно не навешивали, фраер? Гы-гы-гы-гы!".to_string()),
        ]
    );

    let mut fight = DialogStick::from(1);

    fight.replicas.extend(
        vec![
            Replica::from_text("[Главарь:] ПА-ЦА-НЫЫЫЫ!".to_string()),
        ]
    );

    let mut wtf = DialogStick::from(2);
    wtf.replicas.extend(
        vec![
            Replica::from_text("[Главарь:] Чё? Какие еще пельмени?".to_string()),
        ]
    );

    let mut aaa_blond = DialogStick::from(3);
    aaa_blond.replicas.extend(
        vec![
            Replica::from_text("Ааааа, того блондина!? Хахахахахахаха!".to_string()),
            Replica::from_text("Да этот уморыш проиграл их нам в нарды! Мы не забирали их у него".to_string()),
        ]
    );

    let mut he_attack_us = DialogStick::from(4);
    he_attack_us.replicas.extend(
        vec![
            Replica::from_text("Конечно избили, ведь он после проигрыша взбеился и начал виздать как баба!\n\
            Нарды нам раскидал, а затем и вовсе взял палку и набросился! Ну и приподали мы ему урок.\
            \nЧем докажем? А спроси у него самого о оставшихся у него его двух пачек пельмений, которые мы не забрали!\n\
            Мы оставили у себя только то, что честно выиграли. А его добро не брали.".to_string()),
        ]
    );

    let mut go = DialogStick::from(5);
    go.replicas.extend(
        vec![
            Replica::from_text("Вот увидишь, мы не чешим.".to_string()),
        ]
    );

    he_attack_us.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Ляднё, я проверю.".to_string(),
                go.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GOPNIKS_DIALOG_ASK_BLOND)),
            ),
            Variant::create_with_effect(
                "Я не верю, ето чущь!!!".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GOPNIKS_DIALOG_FIGHT)),
            ),
        ],
    }
    );

    aaa_blond.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Чем ви докажете? Вы его избили!".to_string(),
                he_attack_us.id,
                DialogEffect::ReplaceDialog,
            ),
        ],
    }
    );

    wtf.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Котёрые вы забрали у тёго блондиня!".to_string(),
                aaa_blond.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Ай, вбестольку говорить [Напасть]".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GOPNIKS_DIALOG_FIGHT)),
            ),
        ],
    }
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Отдавайте пельмени!".to_string(),
                wtf.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Я присёль свернуть вам щеи.".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GOPNIKS_DIALOG_FIGHT)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(fight.id, fight);
    pool.insert(wtf.id, wtf);
    pool.insert(aaa_blond.id, aaa_blond);
    pool.insert(he_attack_us.id, he_attack_us);
    pool.insert(go.id, go);

    return (root_id, pool);
}

//      START
//        *
//        |
//        ^
//        |
//        *
//       END

pub const BLOND_GIVE_DUMPLINGS_DIALOG: usize = 6;

pub const END_DIALOG_BLOND_GIVE_DUMPLINGS_COMPLETED: usize = 1;

pub fn blond_give_dumplings_dialog() -> Dialog {
    let (root_id, sticks) = blond_give_dumplings();
    return Dialog::from(
        DialogId(BLOND_GIVE_DUMPLINGS_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn blond_give_dumplings() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Здравствуйте, сударъ. Ну как ваши успехи? Вы принесли мне мои пельмени?".to_string()),
        ]
    );

    let mut final_word = DialogStick::from(1);

    final_word.replicas.extend(
        vec![
            Replica::from_text("Ооооо, сударъ, спасибо большое! А что с гопниками? А хотя не важно. Вам полагается, награла!\n\
            Одна па... Ну, то есть ноль... А, хотя, половина пачк... А, ладно, ладно, одна пачка пельмений.".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Дя, вот они.".to_string(),
                final_word.id,
                DialogEffect::EndDialog(Some(END_DIALOG_BLOND_GIVE_DUMPLINGS_COMPLETED)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(final_word.id, final_word);

    return (root_id, pool);
}


//      START
//        *
//        |
//        ^
//        |
//        *
//       | |
//       ^ ^
//       | |
//       * * -> END
//      | |
//      ^ ^
//      | |
//      * * -> END

pub const BLOND_TAKE_DUMPLINGS_DIALOG: usize = 7;

pub const END_DIALOG_BLOND_TAKE_DUMPLINGS_JUST_COMPLETED: usize = 1;
pub const END_DIALOG_BLOND_TAKE_DUMPLINGS_NECK_TWISTED: usize = 2;

pub fn blond_take_dumplings_dialog() -> Dialog {
    let (root_id, sticks) = blond_take_dumplings();
    return Dialog::from(
        DialogId(BLOND_GIVE_DUMPLINGS_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn blond_take_dumplings() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Здравствуйте, сударъ. Ну как ваши успехи? Вы принесли мне мои пельмени?".to_string()),
        ]
    );

    let mut no_it_is_not_me = DialogStick::from(1);

    no_it_is_not_me.replicas.extend(
        vec![
            Replica::from_text("Да как вы... Сударь! Так это они вас обманули, как я мог напасть!? Не верьте этим прохиндеям!".to_string()),
        ]
    );

    let mut truth = DialogStick::from(2);

    truth.replicas.extend(
        vec![
            Replica::from_text("Хорошо, хорошо, я тогда погорячился и напал на них. А что? Я то побит в итоге. Я жерва!\n\
            И пельмени эти все равно мои! Живо метнись и верне мне их!".to_string()),
        ]
    );

    let mut i_go = DialogStick::from(3);

    i_go.replicas.extend(
        vec![
            Replica::from_text("Нуу, сударъ, куда вы! Сударъ..!".to_string()),
        ]
    );

    let mut neck_twisted = DialogStick::from(4);

    neck_twisted.replicas.extend(
        vec![
            Replica::from_text("Сударъ, не..!.. Ээээуууу.".to_string()),
        ]
    );

    truth.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "[Свернуть шею] ЫЫЫЫЫУУУУ!!!! Не указивай мне!!!".to_string(),
                neck_twisted.id,
                DialogEffect::EndDialog(Some(END_DIALOG_BLOND_TAKE_DUMPLINGS_NECK_TWISTED)),
            ),
            Variant::create_with_effect(
                "[Уйти] Сям заибряй свои пельмени!!!".to_string(),
                i_go.id,
                DialogEffect::EndDialog(Some(END_DIALOG_BLOND_TAKE_DUMPLINGS_JUST_COMPLETED)),
            ),
        ],
    }
    );

    no_it_is_not_me.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Говори правдю, не тё щею сверню! Ты прятесь за пазухой другие две пачки пельмений, дя?".to_string(),
                truth.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "[Уйти] Ай, cям забиряй свои пельмени.".to_string(),
                i_go.id,
                DialogEffect::EndDialog(Some(END_DIALOG_BLOND_TAKE_DUMPLINGS_JUST_COMPLETED)),
            ),
        ],
    }
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Ты обманюль меня! Это ты напал на гопников сам, после того как проиграль в нарди свои пельмени!".to_string(),
                no_it_is_not_me.id,
                DialogEffect::ReplaceDialog,
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(no_it_is_not_me.id, no_it_is_not_me);
    pool.insert(truth.id, truth);
    pool.insert(i_go.id, i_go);
    pool.insert(neck_twisted.id, neck_twisted);

    return (root_id, pool);
}

//      START
//        *
//        |
//        *
//        |
// -> ^ ^ ^ ^ ^
// |  | | | | |
//  --*-*-* * * -> END
//          |
//          *
//   -> ^ ^ ^ ^ ^ ^
//   |  | | | | | |
//    --*-*-* * * * -> END
//            | |
//            END

pub const GUARDIAN_FIRST_DIALOG: usize = 8;

pub const END_DIALOG_GUARDIAN_FIRST_JUST_COMPLETED: usize = 1;
pub const END_DIALOG_GUARDIAN_FIRST_BEATEN: usize = 2;
pub const END_DIALOG_GUARDIAN_FIRST_DREVNIRA_STOP_ACCEPTED: usize = 3;

pub fn guardian_first_dialog() -> Dialog {
    let (root_id, sticks) = guardian_first();
    return Dialog::from(
        DialogId(GUARDIAN_FIRST_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn guardian_first() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("[Перед тобой стоят двое охранников с дубинками. Откуда-то слышно монотонное повторение \"Перро, дай мне перо\",\n\
            и одного из охранников на каждую фразу дергается веко. Второй охранник поворачивается к тебе и говорит...]".to_string()),
            Replica::from_text("Прохода нет!".to_string()),
        ]
    );

    let mut is_it_court = DialogStick::from(1);

    is_it_court.replicas.extend(
        vec![
            Replica::from_text("Дааа, даааа, это здание суда. Скоро как раз начнется закрытое заседание.".to_string()),
        ]
    );

    let mut why_i_cant = DialogStick::from(2);

    why_i_cant.replicas.extend(
        vec![
            Replica::from_text("Нам приказано не кого не пускать, потому что это закрытое заседание.".to_string()),
        ]
    );

    let mut i_have_docs = DialogStick::from(3);

    i_have_docs.replicas.extend(
        vec![
            Replica::from_text("В самом деле? Ну, что же вы опаздываете? Вы должны были прийти за 2 часа до начала. Заседание начнется уже через час.\n\
            Наверное, без вас. Мы вас, не пустим, приказ есть приказ.".to_string()),
        ]
    );

    let mut fight = DialogStick::from(4);

    fight.replicas.extend(
        vec![
            Replica::from_text("[Другой охранник:] Хех, как раз кости разомнем!".to_string()),
        ]
    );

    let mut but_i_need = DialogStick::from(5);

    but_i_need.replicas.extend(
        vec![
            Replica::from_text("[Другой охранник шепнул тому, с кем ты говоришь:] Слушай, а может это... Попросим его об этом деле?".to_string()),
            Replica::from_text("Ааа, черт, давай. Короче, гражданин, у нас к тебе дело. Заткни вон ту конченную старужку,\
            которая все трындит со свои \"Перо\"\nА мы тебя тогда, так уж и быть, пропустим, нарушим приказ, что уж... Что скажешь?".to_string()),
        ]
    );

    let mut who_is_old_woman = DialogStick::from(6);

    who_is_old_woman.replicas.extend(
        vec![
            Replica::from_text("Её зовут Древрина. Она сумасшедшая и с ней невозможно вступить в диалог. Сам увидишь.".to_string()),
        ]
    );

    let mut what_she_does = DialogStick::from(7);

    what_she_does.replicas.extend(
        vec![
            Replica::from_text("Ну ты слышешь это \"Пьерро, дай мне перо\"!? А вот представь какого нам тут стоять и слушать это постоянно!\n\
            У моего товарища уже нервный тик началася! Постой с нами хотя-бы час и все поймешь.".to_string()),
        ]
    );

    let mut where_is_she = DialogStick::from(8);

    where_is_she.replicas.extend(
        vec![
            Replica::from_text("[Другой охранник:] О, её легко найти: просто идешь на зву \"перо, перо\". Иди вдоль здания на запад и звук усилится.\n\
            Там увидишь её".to_string()),
        ]
    );

    let mut i_will_do = DialogStick::from(9);

    i_will_do.replicas.extend(
        vec![
            Replica::from_text("Да, просим тебя. Сделай так, чтобы она заткнулась.".to_string()),
        ]
    );

    let mut i_will_complain = DialogStick::from(10);

    i_will_complain.replicas.extend(
        vec![
            Replica::from_text("Эээээ... Ладно, а можешь ты не скажешь никому, а мы тебя пропустим? Проходи.".to_string()),
        ]
    );

    but_i_need.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create(
                "Кто ета старушка?".to_string(),
                who_is_old_woman.id,
            ),
            Variant::create(
                "Чем она вам конкретно мещает?".to_string(),
                what_she_does.id,
            ),
            Variant::create(
                "Где она находиться?".to_string(),
                where_is_she.id,
            ),
            Variant::create_with_effect(
                "Лядно, я посмотрю, что можно сделать.".to_string(),
                i_will_do.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_FIRST_DREVNIRA_STOP_ACCEPTED)),
            ),
            Variant::create_with_effect(
                "[Языкастость] А может я просто расскажу как вы превышаете полномочия, да еще и вовлекаете гражданьских лиц?".to_string(),
                i_will_complain.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_FIRST_JUST_COMPLETED)),
            ),
            Variant::create_with_effect(
                "[Атаковать] Я не собираюсь иметь делё с ней, лучше вам щеи сверню!".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_FIRST_BEATEN)),
            ),
        ],
    }
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create(
                "Ето же здание суда?".to_string(),
                is_it_court.id,
            ),
            Variant::create(
                "Ыыыу, почему нельзя пройти!?".to_string(),
                why_i_cant.id,
            ),
            Variant::create(
                "У мнея повестка в сюд!".to_string(),
                i_have_docs.id,
            ),
            Variant::create_with_effect(
                "Ето нелепо!. Мне нюжно пройти сюда!".to_string(),
                but_i_need.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "[Атаковать] Ыыыыу!!! Вон с моей дороги!".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_FIRST_BEATEN)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(is_it_court.id, is_it_court);
    pool.insert(why_i_cant.id, why_i_cant);
    pool.insert(i_have_docs.id, i_have_docs);
    pool.insert(fight.id, fight);
    pool.insert(but_i_need.id, but_i_need);
    pool.insert(who_is_old_woman.id, who_is_old_woman);
    pool.insert(what_she_does.id, what_she_does);
    pool.insert(where_is_she.id, where_is_she);
    pool.insert(i_will_do.id, i_will_do);
    pool.insert(i_will_complain.id, i_will_complain);

    return (root_id, pool);
}


//      START
//        *
//        |
//      ^ ^ ^
//      | | |
//      * * *
//       END

pub const GUARDIAN_SECOND_DIALOG: usize = 9;

pub const END_DIALOG_GUARDIAN_SECOND_COMPLETED: usize = 1;
pub const END_DIALOG_GUARDIAN_SECOND_BEATEN: usize = 2;


pub fn guardian_second_dialog() -> Dialog {
    let (root_id, sticks) = guardian_second();
    return Dialog::from(
        DialogId(GUARDIAN_SECOND_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn guardian_second() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Ну как, ты разделался со старухой?".to_string()),
        ]
    );

    let mut not_yet = DialogStick::from(1);

    not_yet.replicas.extend(
        vec![
            Replica::from_text("Поторопись, а то мы с ума сойдем.".to_string()),
        ]
    );

    let mut i_will_complain = DialogStick::from(2);

    i_will_complain.replicas.extend(
        vec![
            Replica::from_text("Эээээ... Ладно, а можешь ты не скажешь никому, а мы тебя пропустим? Проходи.".to_string()),
        ]
    );

    let mut fight = DialogStick::from(3);

    fight.replicas.extend(
        vec![
            Replica::from_text("Ряяяяя!!! Говнюк!!!".to_string()),
        ]
    );


    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Еще неть.".to_string(),
                not_yet.id,
                DialogEffect::EndDialog(None),
            ),
            Variant::create_with_effect(
                "[Языкастость] А может я просто расскажу как вы превышаете полномочия, да еще и вовлекаете гражданьских лиц?".to_string(),
                i_will_complain.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_SECOND_COMPLETED)),
            ),
            Variant::create_with_effect(
                "[Атаковать] Я передумал. Пожалуй, я простё сверню вам щеи!".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_SECOND_BEATEN)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(not_yet.id, not_yet);
    pool.insert(i_will_complain.id, i_will_complain);
    pool.insert(fight.id, fight);

    return (root_id, pool);
}


//      START
//        *
//        |
//        *
//        |
//      | | |
//      ^ ^ ^
//      | | |
//      * * *
//       END
pub const GUARDIAN_THIRD_DIALOG: usize = 10;

pub const END_DIALOG_GUARDIAN_THIRD_COMPLETED: usize = 1;
pub const END_DIALOG_GUARDIAN_THIRD_BEATEN: usize = 2;

pub fn guardian_third_dialog() -> Dialog {
    let (root_id, sticks) = guardian_third();
    return Dialog::from(
        DialogId(GUARDIAN_THIRD_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn guardian_third() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("[На лицах охранников блаженство. Они улыбнулись тебе, когда ты подошел, и один из них сказал...]".to_string()),
            Replica::from_text("Ох, спасибо тебе огромное! Эта старуха нас так достала своими монотонными речами, ужас!\
            \nИ мы видили как ты с ней разделался, ха! Можете свободно проходить!".to_string()),
        ]
    );

    let mut you_can_go = DialogStick::from(1);
    you_can_go.replicas.extend(
        vec![
            Replica::from_text("Давайте, проходите!".to_string()),
        ]
    );

    let mut tease = DialogStick::from(2);
    tease.replicas.extend(
        vec![
            Replica::from_text("РЯЯЯЯЯЯЯЯ!!!!! ТЫ ПОЛУЧИШЬ!".to_string()),
        ]
    );

    let mut fight = DialogStick::from(3);

    fight.replicas.extend(
        vec![
            Replica::from_text("Ах ты, говнюк!".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Ыу.".to_string(),
                you_can_go.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_THIRD_COMPLETED)),
            ),
            Variant::create_with_effect(
                "[Спровацировать] Пьерро, дай мне перо, перо...".to_string(),
                tease.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_THIRD_BEATEN)),
            ),
            Variant::create_with_effect(
                "[Атаковать] А я вамь все равнё сеи посворачиваю.".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(END_DIALOG_GUARDIAN_THIRD_BEATEN)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(you_can_go.id, you_can_go);
    pool.insert(tease.id, tease);
    pool.insert(fight.id, fight);

    return (root_id, pool);
}

//      START
//        *
//        |
//        *
//        |
//       | |
//       ^ ^
//       | |
//       * *
//       END
pub const HALL_GUARDIAN_FIRST_DIALOG: usize = 11;

pub const HALL_GUARDIAN_FIRST_DIALOG_COMPLETED: usize = 1;
pub const HALL_GUARDIAN_FIRST_DIALOG_BEATEN: usize = 2;

pub fn hall_guardian_first_dialog() -> Dialog {
    let (root_id, sticks) = hall_guardian_first();
    return Dialog::from(
        DialogId(HALL_GUARDIAN_FIRST_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn hall_guardian_first() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("[Перед тобой двое мощных охранников.]".to_string()),
            Replica::from_text("Вход пока закрыт. Жди.".to_string()),
        ]
    );

    let mut just_wait = DialogStick::from(1);
    just_wait.replicas.extend(
        vec![
            Replica::from_text("Подождите немного. Побродите, пообщайтесь.".to_string()),
        ]
    );

    let mut fight = DialogStick::from(2);

    fight.replicas.extend(
        vec![
            Replica::from_text("Ах ты, говнюк!".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Но мене надо войти!".to_string(),
                just_wait.id,
                DialogEffect::EndDialog(Some(HALL_GUARDIAN_FIRST_DIALOG_COMPLETED)),
            ),
            Variant::create_with_effect(
                "[Атаковать] Вон с дороги!!!".to_string(),
                fight.id,
                DialogEffect::EndDialog(Some(HALL_GUARDIAN_FIRST_DIALOG_BEATEN)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(just_wait.id, just_wait);
    pool.insert(fight.id, fight);

    return (root_id, pool);
}


//      START
//        *
//        |
//        ^
//        |
//        *
//       END
pub const HALL_GUARDIAN_SECOND_DIALOG: usize = 12;

pub const HALL_GUARDIAN_SECOND_COMPLETED: usize = 1;

pub fn hall_guardian_second_dialog() -> Dialog {
    let (root_id, sticks) = hall_guardian_second();
    return Dialog::from(
        DialogId(HALL_GUARDIAN_SECOND_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn hall_guardian_second() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Проходи, не задерживайся.".to_string()),
        ]
    );

    let mut what = DialogStick::from(1);
    what.replicas.extend(
        vec![
            Replica::from_text("...".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Чё?".to_string(),
                what.id,
                DialogEffect::EndDialog(Some(HALL_GUARDIAN_SECOND_COMPLETED)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(what.id, what);

    return (root_id, pool);
}

//      START
//        *
//        |
//      ^ ^ ^
//      | | |
//      * * *
//       END
pub const TABLE_1_DIALOG: usize = 13;

pub const TABLE_1_COMPLETED: usize = 1;

pub fn table_1_dialog() -> Dialog {
    let (root_id, sticks) = table_1();
    return Dialog::from(
        DialogId(TABLE_1_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn table_1() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Сударь, я не занимаюсь делами посетителей, обратитесь к столику с номером 2.".to_string()),
        ]
    );

    let mut where_second = DialogStick::from(1);
    where_second.replicas.extend(
        vec![
            Replica::from_text("Справа от меня.".to_string()),
        ]
    );

    let mut get_out_here = DialogStick::from(2);
    get_out_here.replicas.extend(
        vec![
            Replica::from_text("Еще раз, я не занимаюсь посетителями, обратитесь к столику 2, сударь!".to_string()),
        ]
    );

    let mut go_go = DialogStick::from(3);

    go_go.replicas.extend(
        vec![
            Replica::from_text("...".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Где столик номер 2?".to_string(),
                where_second.id,
                DialogEffect::EndDialog(Some(TABLE_1_COMPLETED)),
            ),
            Variant::create_with_effect(
                "Да я только спросить...".to_string(),
                get_out_here.id,
                DialogEffect::EndDialog(Some(TABLE_1_COMPLETED)),
            ),
            Variant::create_with_effect(
                "Ыу, мне все равнё рожя твоя не понравилясь.".to_string(),
                go_go.id,
                DialogEffect::EndDialog(Some(TABLE_1_COMPLETED)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(where_second.id, where_second);
    pool.insert(get_out_here.id, get_out_here);
    pool.insert(go_go.id, go_go);

    return (root_id, pool);
}

//      START
//        *
//        |
//       ^ ^
//       | |
//       * *
//       END
pub const TABLE_2_DIALOG: usize = 14;

pub const TABLE_2_COMPLETED: usize = 1;

pub fn table_2_dialog() -> Dialog {
    let (root_id, sticks) = table_2();
    return Dialog::from(
        DialogId(TABLE_3_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn table_2() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Сударь, я не занимаюсь посетителями. Обратитесь к столику номер 3.".to_string()),
        ]
    );

    let mut where_second = DialogStick::from(1);
    where_second.replicas.extend(
        vec![
            Replica::from_text("Вон, справа от меня.".to_string()),
        ]
    );

    let mut go_go = DialogStick::from(3);

    go_go.replicas.extend(
        vec![
            Replica::from_text("Хм.".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Ну где столик номер 3?".to_string(),
                where_second.id,
                DialogEffect::EndDialog(Some(TABLE_2_COMPLETED)),
            ),
            Variant::create_with_effect(
                "Ыу, подозрительно.".to_string(),
                go_go.id,
                DialogEffect::EndDialog(Some(TABLE_2_COMPLETED)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(where_second.id, where_second);
    pool.insert(go_go.id, go_go);

    return (root_id, pool);
}


//      START
//        *
//        |
//       ^ ^
//       | |
//       * *
//       END

pub const TABLE_3_DIALOG: usize = 15;

pub const TABLE_3_COMPLETED: usize = 1;

pub fn table_3_dialog() -> Dialog {
    let (root_id, sticks) = table_3();
    return Dialog::from(
        DialogId(TABLE_2_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn table_3() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Здравствуйте. Что вы хотели?".to_string()),
        ]
    );

    let mut go_to_first = DialogStick::from(1);
    go_to_first.replicas.extend(
        vec![
            Replica::from_text("А. Ну, я не занимаюсь такими вопросами, обратитесь к столику с номером 1".to_string()),
        ]
    );

    let mut how_you_guess = DialogStick::from(3);

    how_you_guess.replicas.extend(
        vec![
            Replica::from_text("Хмм, как вы угадали? Я действительно не занимаюсь. Такими вопросами. Обратиесь к столику номер 1.".to_string()),
        ]
    );

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Когда меня пустять в заль судя?".to_string(),
                go_to_first.id,
                DialogEffect::EndDialog(Some(TABLE_3_COMPLETED)),
            ),
            Variant::create_with_effect(
                "Дяй угадяю: ти не занимаесся вопросями посетителей?".to_string(),
                how_you_guess.id,
                DialogEffect::EndDialog(Some(TABLE_3_COMPLETED)),
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(go_to_first.id, go_to_first);
    pool.insert(how_you_guess.id, how_you_guess);

    return (root_id, pool);
}

//       START
//         *
//         |
//         *
//         |
//    ^----------^
//    |          |
//    *          *
//    |        |   |
//    *<-------^   ^
//    |            |
// ^ ^ ^ ^ ^       *
//   |     |      | |
//   *     *      ^ ^
//  | |<-  |      | |
//  ^ ^ |  *      * *
//  | | ^ ^ ^ ^ ^ END
//  * * |       |
//  END --|     *
//        |     |
//        |     *
//         ^ ^ ^ ^ ^ ^
//                 |
//                 *
//                | |
//                ^ ^
//                | |
//                * *
//                END

pub const CRAZY_MAN_DIALOG: usize = 16;

pub const CRAZY_MAN_DIALOG_COMPLETED: usize = 1;
pub const CRAZY_MAN_DIALOG_BEATEN: usize = 2;

pub fn crazy_man_dialog() -> Dialog {
    let (root_id, sticks) = crazy_man();
    return Dialog::from(
        DialogId(CRAZY_MAN_DIALOG),
        "Dialog 1".to_string(),
        "background/dialog_bg.png".to_string(),
        "npc/dialog_courier.png".to_string(),
        root_id,
        sticks,
    );
}

fn crazy_man() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);
    main_stick.replicas.extend(
        vec![
            Replica::from_text("Ххха! Вижу тебе не чем заняться? Не хочешь скоротать время?".to_string()),
            Replica::from_text("У меня есть несколько загадок, не хочешь поодгадывать?".to_string()),
        ]
    );

    let mut guess_start = DialogStick::from(1);
    guess_start.replicas.extend(
        vec![
            Replica::from_text("Хххааа! Ну давай, давай, посмотрим на сколько ты умен!".to_string()),
            Replica::from_text("В рот его мне надо взять,\nЧтобы смачно пососать.\nСладко стало наконец,\nВедь во рту он,...".to_string()),
        ]
    );


    let mut wrong_answer = DialogStick::from(2);
    wrong_answer.replicas.extend(
        vec![
            Replica::from_text("Хахахахахах! Вот, ты ишак! Неправльный ответ, лалка ты!".to_string()),
        ]
    );

    let mut go_get_out_here = DialogStick::from(3);
    go_get_out_here.replicas.extend(
        vec![
            Replica::from_text("Вали, вали. Ххааа!".to_string()),
        ]
    );


    let mut figth = DialogStick::from(4);
    figth.replicas.extend(
        vec![
            Replica::from_text("Ххххха! Да я тебе твоя язык вправлю!".to_string()),
        ]
    );

    wrong_answer.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Ай, нафиг твои загадки.".to_string(),
                go_get_out_here.id,
                DialogEffect::EndDialog(Some(CRAZY_MAN_DIALOG_COMPLETED)),
            ),
            Variant::create_with_effect(
                "ЫЫЫЫУУУУ, щею свреню!!!".to_string(),
                figth.id,
                DialogEffect::EndDialog(Some(CRAZY_MAN_DIALOG_BEATEN)),
            ),
        ],
    }
    );

    let mut right_answer_1 = DialogStick::from(5);
    right_answer_1.replicas.extend(
        vec![
            Replica::from_text("Хм. Верно! Возможно, ты не такой тупой, как на первый взляд! Следующая...".to_string()),
            Replica::from_text("Эти круглые две штучки\nВсех притягивают взор,\nИми мы весь мир увидим,\nРасширяем кругозор.".to_string()),
        ]
    );

    let mut right_answer_2 = DialogStick::from(6);
    right_answer_2.replicas.extend(
        vec![
            Replica::from_text("Да ты не перестаешь меня удивлять! Правильно! Ну и последняя...".to_string()),
            Replica::from_text("Их тридцать три, плюс один генерал,\nКак бы ты их в три слова назвал?".to_string()),
        ]
    );

    let mut right_answer_3 = DialogStick::from(7);
    right_answer_3.replicas.extend(
        vec![
            Replica::from_text("Да! Это зубы и ЯЗЫК! ТЫК В ЯЗЫК [Показывает пальцев в твой язык, но не дотрагивается.".to_string()),
        ]
    );

    let mut bye = DialogStick::from(8);
    bye.replicas.extend(
        vec![
            Replica::from_text("Ххааа! Давай, покеда.".to_string()),
        ]
    );

    right_answer_3.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Мдя. Я ухожу.".to_string(),
                bye.id,
                DialogEffect::EndDialog(Some(CRAZY_MAN_DIALOG_COMPLETED)),
            ),
            Variant::create_with_effect(
                "[Напасть] И щея твоя КХЫК!".to_string(),
                figth.id,
                DialogEffect::EndDialog(Some(CRAZY_MAN_DIALOG_BEATEN)),
            ),
        ],
    });

    right_answer_2.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "33 ПЕЛЬМЕНЯ И Я".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Зубы и язык".to_string(),
                right_answer_3.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Сольдати?".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Ыыыыу, надоели твои загадки.".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Букви?".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Ну, етё просто: зюбы и рот.".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
        ],
    });

    right_answer_1.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "ПЕЛЬМЕНИ".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Очки!".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Титьки...".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Иди ты!".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Глазя?".to_string(),
                right_answer_2.id,
                DialogEffect::ReplaceDialog,
            ),
        ],
    });

    guess_start.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Не зняю.".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Язык.".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Леденець".to_string(),
                right_answer_1.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Пельмени".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "ЫЫЫЫУУУУ, ето не приличнё!".to_string(),
                wrong_answer.id,
                DialogEffect::ReplaceDialog,
            ),
        ],
    }
    );

    let mut dont_want = DialogStick::from(9);

    dont_want.replicas.extend(
        vec![
            Replica::from_text("А что? Очкушь да? По тебе видно, что ты очкун.".to_string()),
        ]
    );

    let mut just_dont_want = DialogStick::from(10);

    just_dont_want.replicas.extend(
        vec![
            Replica::from_text("Хххаааа! Нет! Ты - ОЧКУН - ЕНОТ ПАЛАСКУН!".to_string()),
        ]
    );

    just_dont_want.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "[Уйти] Да мне пофигь.".to_string(),
                go_get_out_here.id,
                DialogEffect::EndDialog(Some(CRAZY_MAN_DIALOG_COMPLETED)),
            ),
            Variant::create_with_effect(
                "[Напасть] Пфффхеееее....".to_string(),
                figth.id,
                DialogEffect::EndDialog(Some(CRAZY_MAN_DIALOG_BEATEN)),
            ),
        ],
    });

    dont_want.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Ляяядно, давай свои зягадьки.".to_string(),
                guess_start.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Ниеть, просте не хотю!".to_string(),
                just_dont_want.id,
                DialogEffect::ReplaceDialog,
            ),
        ],
    });

    main_stick.branching = Some(Branching {
        id: 0,
        variants: vec![
            Variant::create_with_effect(
                "Ну дявай.".to_string(),
                guess_start.id,
                DialogEffect::ReplaceDialog,
            ),
            Variant::create_with_effect(
                "Нихатю, мне надо на сюд.".to_string(),
                dont_want.id,
                DialogEffect::ReplaceDialog,
            ),
        ],
    }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(guess_start.id, guess_start);
    pool.insert(wrong_answer.id, wrong_answer);
    pool.insert(go_get_out_here.id, go_get_out_here);
    pool.insert(figth.id, figth);
    pool.insert(right_answer_1.id, right_answer_1);
    pool.insert(right_answer_2.id, right_answer_2);
    pool.insert(right_answer_3.id, right_answer_3);
    pool.insert(bye.id, bye);
    pool.insert(dont_want.id, dont_want);
    pool.insert(just_dont_want.id, just_dont_want);

    return (root_id, pool);
}
