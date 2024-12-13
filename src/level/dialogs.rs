use std::collections::HashMap;

use crate::dialog::{Branching, Dialog, DialogEffect, DialogId, DialogStick, Replica, Variant};

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

pub const BLOND_DIALOG_FIRST: usize = 4;

pub const END_DIALOG_BLOND_FIRST_ACCEPTED: usize = 1;
pub const END_DIALOG_BLOND_FIRST_DENIED: usize = 2;

pub fn blond_dialog_first() -> Dialog {
    let (root_id, sticks) = blond_first();
    return Dialog::from(
        DialogId(BLOND_DIALOG_FIRST),
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
