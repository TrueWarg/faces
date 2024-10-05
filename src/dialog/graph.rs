use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct DialogStick {
    pub id: usize,
    replicas: Vec<Replica>,
    branching: Option<Branching>,
}

impl Default for DialogStick {
    fn default() -> Self {
        return DialogStick {
            id: 0,
            replicas: vec![],
            branching: None,
        };
    }
}

impl DialogStick {
    pub fn replicas_size(&self) -> usize {
        return self.replicas.len();
    }

    pub fn first_replica(&self) -> &Replica {
        return self.get_replica_at(0);
    }

    pub fn last_replica(&self) -> &Replica {
        return self.get_replica_at(self.replicas.len() - 1);
    }

    pub fn get_replica_at(&self, idx: usize) -> &Replica {
        return &self.replicas[idx];
    }

    pub fn get_branching(&self) -> &Option<Branching> {
        return &self.branching;
    }

    pub fn from(id: usize) -> Self {
        return DialogStick {
            id,
            replicas: vec![],
            branching: None,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Replica {
    pub text: String,
}

impl Replica {
    pub fn from_text(text: String) -> Replica {
        return Replica {
            text,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Branching {
    pub id: usize,
    pub variants: Vec<Variant>,
}

impl Default for Branching {
    fn default() -> Self {
        return Branching {
            id: 0,
            variants: vec![],
        };
    }
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub label: String,
    pub stick_id: usize,
    pub effect: Option<DialogEffect>,
}

impl Variant {
    fn create_with_effect(label: String, stick_id: usize, effect: DialogEffect) -> Self {
        return Variant {
            label,
            stick_id,
            effect: Some(effect),
        };
    }

    fn create(label: String, stick_id: usize) -> Self {
        return Variant {
            label,
            stick_id,
            effect: None,
        };
    }
}

#[derive(Clone, Debug)]
pub enum DialogEffect {
    ReplaceDialog,
    EndDialog,
}

fn dialog_run_str(dialog: (usize, HashMap<usize, DialogStick>), answers: Vec<usize>) -> String {
    let (root_id, pool) = dialog;
    let mut result = "".to_string();
    let mut branch_stack = vec![root_id];
    let mut answer_number = 0;
    let mut was_in_stack = HashSet::new();
    loop {
        let stick_id = branch_stack.last().expect("");
        let branch = pool.get(stick_id).expect("No value");
        if !was_in_stack.contains(&branch.id) {
            for replica in &branch.replicas {
                result += replica.text.as_str();
                result += "\n\n";
            }
        }
        was_in_stack.insert(branch.id);

        match &branch.branching {
            None => {
                was_in_stack.remove(&branch.id);
                branch_stack.pop();
            }
            Some(branching) => {
                result += "-------------------------\n";
                for (idx, variant) in branching.variants.iter().enumerate() {
                    result += format!("{}. {}\n", idx + 1, variant.label).as_str();
                }
                result += "-------------------------\n\n";
                let answer = answers[answer_number];
                let variant = &branching.variants[answer];
                result += format!("{}\n\n", variant.label).as_str();
                answer_number += 1;
                match &variant.effect {
                    None => {}
                    Some(effect) => {
                        match effect {
                            DialogEffect::ReplaceDialog => {
                                was_in_stack.remove(&branch.id);
                                branch_stack.pop();
                            }
                            DialogEffect::EndDialog => {
                                branch_stack.clear();
                            }
                        }
                    }
                }
                branch_stack.push(variant.stick_id)
            }
        }

        if branch_stack.is_empty() {
            break;
        }
    }

    return result;
}

//         start
//         *
//         |
//         *
//    -->^ ^ ^
//    |  | | |
//    ---* * *
//         end
fn test_dialog_0() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);

    main_stick.replicas.extend(
        vec![
            Replica::from_text("Hello, mu***r!".to_string()),
            Replica::from_text("What do you want?".to_string()),
        ]
    );

    let mut question_stick = DialogStick::from(1);
    question_stick.replicas.push(
        Replica::from_text("My name is RED SPIKE BOY!".to_string())
    );

    let mut neck_twist_stick = DialogStick::from(2);
    neck_twist_stick.replicas.push(
        Replica::from_text("Not well smelt you!".to_string())
    );

    let mut go_away_stick = DialogStick::from(3);
    go_away_stick.replicas.push(
        Replica::from_text("Yes, get out here!".to_string())
    );

    main_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create("What is your name?".to_string(), question_stick.id),
                Variant::create_with_effect("I will twist your neck!".to_string(), neck_twist_stick.id, DialogEffect::ReplaceDialog),
                Variant::create_with_effect("I will go.".to_string(), go_away_stick.id, DialogEffect::ReplaceDialog),
            ],
        }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(question_stick.id, question_stick);
    pool.insert(neck_twist_stick.id, neck_twist_stick);
    pool.insert(go_away_stick.id, go_away_stick);

    return (root_id, pool);
}

//         start
//         *
//         |
//         *
//   --> ^ ^ ^
//   |   | | |
//   |   * * *
//   |   | end
//   |   *
//   |   |
//   |  ^ ^
//   |  | |
//   -- * *
pub fn test_dialog_1() -> (usize, HashMap<usize, DialogStick>) {
    let mut main_stick = DialogStick::from(0);

    main_stick.replicas.extend(
        vec![
            Replica::from_text("Hello, mu***r!".to_string()),
            Replica::from_text("What do you want?".to_string()),
        ]
    );

    let mut question_stick = DialogStick::from(1);
    question_stick.replicas.push(
        Replica::from_text("My name is RED SPIKE BOY!".to_string())
    );
    question_stick.replicas.push(
        Replica::from_text("And what yours?!".to_string())
    );

    let mut tell_name_stick = DialogStick::from(11);
    tell_name_stick.replicas.push(
        Replica::from_text("What a ridicules name!".to_string())
    );
    let mut dont_tell_name_stick = DialogStick::from(12);
    dont_tell_name_stick.replicas.push(
        Replica::from_text("I don't care!".to_string())
    );

    question_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create_with_effect("Formidable Face!".to_string(), tell_name_stick.id, DialogEffect::ReplaceDialog),
                Variant::create_with_effect("I will not tell you!".to_string(), dont_tell_name_stick.id, DialogEffect::ReplaceDialog),
            ],
        }
    );

    let mut neck_twist_stick = DialogStick::from(2);
    neck_twist_stick.replicas.push(
        Replica::from_text("Not well smelt Formidable!".to_string())
    );

    let mut go_away_stick = DialogStick::from(3);
    go_away_stick.replicas.push(
        Replica::from_text("Yes, get out here!".to_string())
    );

    main_stick.branching = Some(
        Branching {
            id: 0,
            variants: vec![
                Variant::create("What is your name?".to_string(), question_stick.id),
                Variant::create_with_effect("I will twist your neck!".to_string(), neck_twist_stick.id, DialogEffect::ReplaceDialog),
                Variant::create_with_effect("I will go.".to_string(), go_away_stick.id, DialogEffect::ReplaceDialog),
            ],
        }
    );

    let mut pool = HashMap::new();
    let root_id = main_stick.id;
    pool.insert(main_stick.id, main_stick);
    pool.insert(tell_name_stick.id, tell_name_stick);
    pool.insert(dont_tell_name_stick.id, dont_tell_name_stick);
    pool.insert(question_stick.id, question_stick);
    pool.insert(neck_twist_stick.id, neck_twist_stick);
    pool.insert(go_away_stick.id, go_away_stick);

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
pub fn test_dialog_2() -> (usize, HashMap<usize, DialogStick>) {
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
                Этё мой дём!!!!! [Свернуть шею]".to_string(), furiously_twist_neck.id, DialogEffect::EndDialog),
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
                Variant::create_with_effect("Хм. Давайте я расписюсь.".to_string(), ok.id, DialogEffect::EndDialog),
                Variant::create_with_effect("[Свернуть шею]".to_string(), twist_neck.id, DialogEffect::EndDialog),
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

#[test]
fn test_dialog_00() {
    let mut expected = "Hello, mu***r!\n\n".to_string();
    expected += "What do you want?\n\n";
    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will twist your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";
    expected += "What is your name?\n\n";
    expected += "My name is RED SPIKE BOY!\n\n";
    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will twist your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";
    expected += "I will twist your neck!\n\n";
    expected += "Not well smelt you!\n\n";

    assert_eq!(expected, dialog_run_str(test_dialog_0(), vec![0, 1]))
}

#[test]
fn test_dialog_01() {
    let mut expected = "Hello, mu***r!\n\n".to_string();
    expected += "What do you want?\n\n";
    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will twist your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";
    expected += "I will twist your neck!\n\n";
    expected += "Not well smelt you!\n\n";

    assert_eq!(expected, dialog_run_str(test_dialog_0(), vec![1]))
}

#[test]
fn test_dialog_10() {
    let mut expected = "Hello, mu***r!\n\n".to_string();
    expected += "What do you want?\n\n";

    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will twist your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";

    expected += "What is your name?\n\n";
    expected += "My name is RED SPIKE BOY!\n\n";
    expected += "And what yours?!\n\n";

    expected += "-------------------------\n";
    expected += "1. Formidable Face!\n";
    expected += "2. I will not tell you!\n";
    expected += "-------------------------\n\n";

    expected += "Formidable Face!\n\n";
    expected += "What a ridicules name!\n\n";

    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will twist your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";

    expected += "I will twist your neck!\n\n";
    expected += "Not well smelt Formidable!\n\n";

    assert_eq!(expected, dialog_run_str(test_dialog_1(), vec![0, 0, 1]))
}

#[test]
fn test_dialog_11() {
    let mut expected = "Hello, mu***r!\n\n".to_string();
    expected += "What do you want?\n\n";
    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will twist your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";
    expected += "I will go.\n\n";
    expected += "Yes, get out here!\n\n";

    assert_eq!(expected, dialog_run_str(test_dialog_1(), vec![2]))
}

#[test]
fn test_dialog_21() {
    let mut expected = "Здравстуйте! У меня к вам очень важное дело.\n\n".to_string();
    expected += "Вам повестка в суд! Вы должны расписаться здесь и здесь, а еще вот тут. Пожалуйста, вот вам ручка.\n\n";
    expected += "Только не забудьте её мне вернуть, хорошо?\n\n";

    expected += "-------------------------\n";
    expected += "1. Ыыыу, ето ошибка! Мне не должно ничего приходить!\n";
    expected += "2. Почему мне присля этя повесьтька?\n";
    expected += "3. Когдя мне надо в суд?\n";
    expected += "4. Хм. Давайте я расписюсь.\n";
    expected += "5. [Свернуть шею]\n";
    expected += "-------------------------\n\n";

    expected += "Почему мне присля этя повесьтька?\n\n";
    expected += "Я просто курьер, особых подробностей не знаю.\n\n";
    expected += "Но вроде как вы объвиняетесь в незаконном владении имуществом, \
    а именно Вашим, или, если обвинения не беспочвенны, не Вашем домом.\n\n";

    expected += "-------------------------\n";
    expected += "1. [Промолчать]\n";
    expected += "2. Ыыыыыыууууу!!!! Этё мой дём!!!!! [Свернуть шею]\n";
    expected += "-------------------------\n\n";

    expected += "Ыыыыыыууууу!!!! Этё мой дём!!!!! [Свернуть шею]\n\n";
    expected += "Что вы... Грозный, ввваааааа! Эу...\n\n";

    assert_eq!(expected, dialog_run_str(test_dialog_2(), vec![1, 1]))
}

#[test]
fn test_dialog_22() {
    let mut expected = "Здравстуйте! У меня к вам очень важное дело.\n\n".to_string();
    expected += "Вам повестка в суд! Вы должны расписаться здесь и здесь, а еще вот тут. Пожалуйста, вот вам ручка.\n\n";
    expected += "Только не забудьте её мне вернуть, хорошо?\n\n";

    expected += "-------------------------\n";
    expected += "1. Ыыыу, ето ошибка! Мне не должно ничего приходить!\n";
    expected += "2. Почему мне присля этя повесьтька?\n";
    expected += "3. Когдя мне надо в суд?\n";
    expected += "4. Хм. Давайте я расписюсь.\n";
    expected += "5. [Свернуть шею]\n";
    expected += "-------------------------\n\n";

    expected += "Почему мне присля этя повесьтька?\n\n";
    expected += "Я просто курьер, особых подробностей не знаю.\n\n";
    expected += "Но вроде как вы объвиняетесь в незаконном владении имуществом, \
    а именно Вашим, или, если обвинения не беспочвенны, не Вашем домом.\n\n";

    expected += "-------------------------\n";
    expected += "1. [Промолчать]\n";
    expected += "2. Ыыыыыыууууу!!!! Этё мой дём!!!!! [Свернуть шею]\n";
    expected += "-------------------------\n\n";
    expected += "[Промолчать]\n\n";
    expected += "Ну так что? Подписывать будете?\n\n";

    expected += "-------------------------\n";
    expected += "1. Ыыыу, ето ошибка! Мне не должно ничего приходить!\n";
    expected += "2. Почему мне присля этя повесьтька?\n";
    expected += "3. Когдя мне надо в суд?\n";
    expected += "4. Хм. Давайте я расписюсь.\n";
    expected += "5. [Свернуть шею]\n";
    expected += "-------------------------\n\n";
    expected += "[Свернуть шею]\n\n";
    expected += "Да что вы...! Эуу....!\n\n";

    assert_eq!(expected, dialog_run_str(test_dialog_2(), vec![1, 0, 4]))
}



pub const INVALID_NODE_ID: usize = 0;