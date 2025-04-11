use iced::widget::{button, checkbox, column, text, Column, row, Row};
use iced::alignment::{Vertical};

mod asm;
mod instruct;
mod m6502;
mod memory;

use crate::asm::{assemble, read_lines};
use crate::m6502::{State, step};
use crate::memory::{DefaultMemory, Memory};

pub fn main() -> iced::Result {
    iced::run("Emultator", Machine::update, Machine::view)
}

struct Machine {
    memory: DefaultMemory,
    state: State,
    curr_page: u8
}

impl Default for Machine {
    fn default() -> Self {
        let state = State::new();

        let mut memory = DefaultMemory::new();

        let lines: Vec<String> = read_lines("example2.asm").unwrap().map(|l| l.unwrap()).collect();
        let res = assemble(lines);

        memory.set(0xFFFC, 0x00);
        memory.set(0xFFFD, 0x06);

        let mut i = 0;
        for val in res {
            memory.set(0x0600 + i, val);
            i += 1;
        }
        Machine {
            state,
            memory,
            curr_page: 0
        }
    }
}

fn page_widget(memory: &dyn Memory, curr_page: u8) -> Column<'_, Message>{
    let mut pageCol = column![
        row![
            button("-10").on_press_maybe(if curr_page >= 0x10 {Some(Message::ChangePage(-0x10))} else {None}),
            button("-1").on_press_maybe(if curr_page >= 0x01 {Some(Message::ChangePage(-0x1))} else {None}),
            text(format!("{:02x}", curr_page)),
            button("+1").on_press_maybe(if curr_page <= (0xFF - 0x01) {Some(Message::ChangePage(0x1))} else {None}),
            button("+10").on_press_maybe(if curr_page <= (0xFF - 0x10) {Some(Message::ChangePage(0x10))} else {None}),
        ].spacing(4).align_y(Vertical::Center),
    ].spacing(4);
    for h in 0x00..0x10 {
        let mut row = Row::new().spacing(4);
        for l in 0x00..0x10 {
            row =  row.push(text(format!("{:02x}", memory.get((h * 0x10) + l + ((curr_page as u16) * 0x0100)))));
        }
        pageCol = pageCol.push(row);
    }
    return pageCol;
}

#[derive(Debug, Clone, Copy)]
enum Message {
    HalfStep,
    Step,
    ChangePage(i8),
    ToggleReset(bool)
}

impl Machine {
    fn update(&mut self, message: Message) {
        match message {
            Message::HalfStep => {
                if self.state.clock1 {
                    step(&mut self.state)
                } else {
                    if self.state.rw {
                        self.state.db = self.memory.get(self.state.ab);
                    } else {
                        self.memory.set(self.state.ab, self.state.db);
                    }
                    step(&mut self.state);
                }
            },
            Message::Step => {
                if self.state.clock1 {
                    step(&mut self.state)
                }
                if self.state.rw {
                    self.state.db = self.memory.get(self.state.ab);
                } else {
                    self.memory.set(self.state.ab, self.state.db);
                }
                step(&mut self.state);
            },
            Message::ToggleReset(is_checked) => {
                self.state.res = is_checked;
            },
            Message::ChangePage(ammount) => {
                self.curr_page = self.curr_page.checked_add_signed(ammount).unwrap();
            }
        }
    }

    fn view(&self) -> Column<Message> {
        println!("{:?}", self.state);
        column![
            row![
                column![
                    text("Program"),
                    text(format!("{:04x}", self.state.registers.pc)),
                ].spacing(10),
                column![
                    text("ACC"),
                    text(format!("{:02x}", self.state.registers.ac)),
                ].spacing(10),
                column![
                    text("X"),
                    text(format!("{:02x}", self.state.registers.xr)),
                ].spacing(10),
                column![
                    text("Y"),
                    text(format!("{:02x}", self.state.registers.yr)),
                ].spacing(10),
                column![
                    text("Status"),
                    text(format!("{:02x}", self.state.registers.sr)),
                ].spacing(10),
                column![
                    text("Stack"),
                    text(format!("{:02x}", self.state.registers.sp)),
                ].spacing(10),
                column![
                    text("Timing"),
                    text(format!("{:?}", self.state.timing)),
                ].spacing(10),
                column![
                    text("Address"),
                    text(format!("{:04x}", self.state.ab)),
                ].spacing(10),
                column![
                    text("Data"),
                    text(format!("{:02x}", self.state.db)),
                ].spacing(10),
            ].spacing(10),
            text(self.state.registers.pc),
            button("Step").on_press_maybe(if self.state.clock1 {Some(Message::Step)} else {None}),
            button("Half step").on_press(Message::HalfStep),
            checkbox("Reset signal", self.state.res).on_toggle(Message::ToggleReset),
            page_widget(&self.memory, self.curr_page)
        ]

    }
}
