use super::{
    BoardState, Players,
    structures::{CollapsedTunnel, Structure, Structures},
    units::{Unit, Units},
};
use crate::app::widgets::CellWidget;
use std::collections::VecDeque;

impl BoardState {
    fn stress_distances(&self) -> Vec<Vec<u8>> {
        let mut distance: Vec<Vec<u8>> = vec![vec![100; self.map_width]; self.map_height];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for (row, cells) in self.cells.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                let source: Option<u8> = match cell.get_structure() {
                    Structures::Base(_) => Some(0),
                    Structures::Tunnel(tunnel) if tunnel.has_support() => Some(20),
                    _ => None,
                };
                if let Some(stress) = source {
                    distance[row][col] = stress;
                    queue.push_back((row, col));
                }
            }
        }
        while let Some((row, col)) = queue.pop_front() {
            let next: u8 = distance[row][col].saturating_add(20).min(100);
            for (r, c) in [
                (row.wrapping_sub(1), col),
                (row + 1, col),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
            ] {
                if r < self.map_height
                    && c < self.map_width
                    && next < distance[r][c]
                    && matches!(
                        self.cells[r][c].get_structure(),
                        Structures::Tunnel(_) | Structures::Base(_) | Structures::Mine(_)
                    )
                {
                    distance[r][c] = next;
                    queue.push_back((r, c));
                }
            }
        }
        distance
    }

    pub fn prospective_tunnel_stress(&self, (row, col): (usize, usize)) -> u8 {
        let distance: Vec<Vec<u8>> = self.stress_distances();
        [
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ]
        .into_iter()
        .filter_map(|(r, c): (usize, usize)| -> Option<u8> {
            distance
                .get(r)
                .and_then(|line: &Vec<u8>| -> Option<&u8> { line.get(c) })
                .copied()
        })
        .min()
        .unwrap_or(100)
        .saturating_add(20)
        .min(100)
    }

    pub fn recalculate_stress(&mut self) -> () {
        let distance: Vec<Vec<u8>> = self.stress_distances();
        for (row, cells) in self.cells.iter_mut().enumerate() {
            for (col, cell) in cells.iter_mut().enumerate() {
                if let Structures::Tunnel(mut tunnel) = cell.get_structure() {
                    tunnel.set_stress(distance[row][col]);
                    cell.set_structure(Structures::Tunnel(tunnel));
                }
            }
        }
    }

    pub fn support_worker(
        &self,
        (row, col): (usize, usize),
        owner: Players,
    ) -> Option<(usize, usize)> {
        if owner == Players::Unclaimed {
            return None;
        }
        [
            (row, col),
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ]
        .into_iter()
        .find(|&(r, c): &(usize, usize)| -> bool {
            self.cells
                .get(r)
                .and_then(|line: &Vec<CellWidget>| -> Option<&CellWidget> { line.get(c) })
                .and_then(|cell: &CellWidget| -> Option<&Units> {
                    cell.get_ref_option_unit().as_ref()
                })
                .is_some_and(|unit: &Units| -> bool {
                    unit.get_owner() == owner && unit.get_can_build()
                })
        })
    }

    pub fn working_on_support(&self, (row, col): (usize, usize)) -> bool {
        [
            (row, col),
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ]
        .into_iter()
        .any(|(r, c): (usize, usize)| -> bool {
            self.cells
                .get(r)
                .and_then(|line: &Vec<CellWidget>| -> Option<&CellWidget> { line.get(c) })
                .is_some_and(|cell: &CellWidget| -> bool {
                    if let Structures::Tunnel(tunnel) = cell.get_structure()
                        && let Some(work) = tunnel.support_work()
                    {
                        self.support_worker((r, c), work.owner) == Some((row, col))
                    } else {
                        false
                    }
                })
        })
    }

    pub fn resolve_collapses(&mut self) -> usize {
        let mut total: usize = 0;
        loop {
            self.recalculate_stress();
            let mut collapsed: usize = 0;
            for row in &mut self.cells {
                for cell in row {
                    if matches!(cell.get_structure(), Structures::Tunnel(tunnel) if tunnel.get_stress() >= 100)
                    {
                        cell.set_structure(Structures::CollapsedTunnel(CollapsedTunnel::new()));
                        cell.get_mut_option_unit().take();
                        collapsed += 1;
                    }
                }
            }
            total += collapsed;
            if collapsed == 0 {
                break;
            }
        }
        if self.marked_cells.marking_cells {
            let (row, col): (usize, usize) = self.marked_cells.selected_unit.get_coords();
            if self.cells[row][col].get_ref_option_unit().is_none() {
                self.marked_cells.marking_cells = false;
                self.end_marking_cells(true);
            }
        }
        total
    }
}
