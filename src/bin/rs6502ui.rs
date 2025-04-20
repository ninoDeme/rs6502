use iced::alignment::Vertical;
use iced::widget::{button, checkbox, column, container, row, scrollable, text, Column, Row};
use iced::Color;

use rs6502::asm::{assemble, read_lines};
use rs6502::m6502::{step, State};
use rs6502::memory::{DefaultMemory, Memory};

use std::rc::Rc;

pub fn main() -> iced::Result {
    iced::run("Emultator", Machine::update, Machine::view)
}

struct Machine {
    memory: DefaultMemory,
    state: State,
    last_states: Vec<Rc<State>>,
    curr_page: u8,
    follow_ab: bool,
    follow_pc: bool,
}

impl Default for Machine {
    fn default() -> Self {
        let state = State::new();

        let mut memory = DefaultMemory::new();

        let lines: Vec<String> = read_lines("example2.asm")
            .unwrap()
            .map(|l| l.unwrap())
            .collect();
        let res = assemble(lines);

        for (key, val) in res {
            memory.set(key, val);
        }
        Machine {
            state,
            last_states: Vec::new(),
            memory,
            curr_page: 0,
            follow_ab: false,
            follow_pc: false,
        }
    }
}

fn page_widget(memory: &dyn Memory, curr_page: u8, ab: u16, pc: u16) -> Column<'_, Message> {
    let mut page_col = column![row![
        button("-10").on_press_maybe(if curr_page >= 0x10 {
            Some(Message::ChangePage(-0x10))
        } else {
            None
        }),
        button("-1").on_press_maybe(if curr_page >= 0x01 {
            Some(Message::ChangePage(-0x1))
        } else {
            None
        }),
        text(format!("{:02x}", curr_page)),
        button("+1").on_press_maybe(if curr_page <= (0xFF - 0x01) {
            Some(Message::ChangePage(0x1))
        } else {
            None
        }),
        button("+10").on_press_maybe(if curr_page <= (0xFF - 0x10) {
            Some(Message::ChangePage(0x10))
        } else {
            None
        }),
    ]
    .spacing(4)
    .align_y(Vertical::Center),]
    .spacing(4);
    for h in 0x00..0x10 {
        let mut row = Row::new().spacing(4);
        for l in 0x00..0x10 {
            let add = (h * 0x10) + l + ((curr_page as u16) * 0x0100);
            let mut container = container(text(format!("{:02x}", memory.get(add))));
            if add == pc {
                if pc == ab {
                    container = container
                        .style(|_| container::background(Color::from_rgba8(255, 150, 255, 1.0)))
                } else {
                    container = container
                        .style(|_| container::background(Color::from_rgba8(200, 255, 200, 1.0)))
                }
            } else if add == ab {
                container = container
                    .style(|_| container::background(Color::from_rgba8(255, 255, 150, 1.0)))
            }
            row = row.push(container);
        }
        page_col = page_col.push(row);
    }
    return page_col;
}

#[derive(Debug, Clone, Copy)]
enum Message {
    HalfStep,
    Step,
    ClearStates,
    ChangePage(i8),
    ToggleReset(bool),
    ToggleFollowAB(bool),
    ToggleFollowPC(bool),
}

impl Machine {
    fn update(&mut self, message: Message) {
        match message {
            Message::HalfStep => {
                if self.state.clock1 {
                    self.last_states.push(self.state.clone().into());
                    step(&mut self.state)
                } else {
                    if self.state.rw {
                        self.state.db = self.memory.get(self.state.ab);
                    } else {
                        self.memory.set(self.state.ab, self.state.db);
                    }
                    self.last_states.push(self.state.clone().into());
                    step(&mut self.state);
                }
            }
            Message::Step => {
                self.last_states.push(self.state.clone().into());
                if self.state.clock1 {
                    step(&mut self.state)
                }
                if self.state.rw {
                    self.state.db = self.memory.get(self.state.ab);
                } else {
                    self.memory.set(self.state.ab, self.state.db);
                }
                self.last_states.push(self.state.clone().into());
                step(&mut self.state);
            }
            Message::ClearStates => {
                self.last_states.clear();
            }
            Message::ToggleReset(is_checked) => {
                self.state.res = is_checked;
            }
            Message::ToggleFollowAB(is_checked) => {
                self.follow_ab = is_checked;
                self.follow_pc = false;
            }
            Message::ToggleFollowPC(is_checked) => {
                self.follow_ab = false;
                self.follow_pc = is_checked;
            }
            Message::ChangePage(ammount) => {
                self.curr_page = self.curr_page.checked_add_signed(ammount).unwrap();
            }
        };

        if self.follow_ab {
            self.curr_page = ((self.state.ab & 0xFF00) >> 8) as u8
        } else if self.follow_pc {
            self.curr_page = ((self.state.registers.pc & 0xFF00) >> 8) as u8
        }
    }

    fn view(&self) -> Row<Message> {
        println!("{:?}", self.state);
        row![
            column![
                row![
                    button("Step").on_press_maybe(if self.state.clock1 {
                        Some(Message::Step)
                    } else {
                        None
                    }),
                    button("Half step").on_press(Message::HalfStep),
                    button("Clear states").on_press(Message::ClearStates),
                ],
                checkbox("Reset signal", self.state.res).on_toggle(Message::ToggleReset),
                row![
                    checkbox("Follow address bus", self.follow_ab)
                        .on_toggle(Message::ToggleFollowAB),
                    checkbox("Follow program counter", self.follow_pc)
                        .on_toggle(Message::ToggleFollowPC),
                ]
                .spacing(4),
                page_widget(
                    &self.memory,
                    self.curr_page,
                    self.state.ab,
                    self.state.registers.pc
                ),
            ],
            scrollable(self.state_table())
        ]
        .spacing(16)
    }
    fn state_table(&self) -> Row<'static, Message> {
        let mut new_vec = self.last_states.clone();
        new_vec.push(self.state.clone().into());
        let input = new_vec.iter();

        const VERT_SPACING: u16 = 4;
        row![
            column![text("Cycle")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{}", state.total_cycles)).into())
                )
                .spacing(VERT_SPACING),
            column![text("Program")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:04x}", state.registers.pc)).into())
                )
                .spacing(VERT_SPACING),
            column![text("Acc")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:02x}", state.registers.ac)).into())
                )
                .spacing(VERT_SPACING),
            column![text("X")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:02x}", state.registers.xr)).into())
                )
                .spacing(VERT_SPACING),
            column![text("Y")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:02x}", state.registers.yr)).into())
                )
                .spacing(VERT_SPACING),
            column![text("Status")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(state.registers.fmt_status()).into())
                )
                .spacing(VERT_SPACING),
            column![text("Stack")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:02x}", state.registers.sp)).into())
                )
                .spacing(VERT_SPACING),
            column![text("State")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:?}", state.timing)).into())
                )
                .spacing(VERT_SPACING),
            column![text("Next")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:?}", state.next_timing)).into())
                )
                .spacing(VERT_SPACING),
            column![text("Address")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:04x}", state.ab)).into())
                )
                .spacing(VERT_SPACING),
            column![text("Data")]
                .extend(
                    input
                        .clone()
                        .map(|state| text(format!("{:02x}", state.db)).into())
                )
                .spacing(VERT_SPACING),
        ]
        .spacing(8)
    }
}
