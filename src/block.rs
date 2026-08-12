use crate::document::Position;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BlockMarkers {
    pub first: Option<Position>,
    pub second: Option<Position>,
}

impl BlockMarkers {
    pub fn set(&mut self, position: Position) {
        if self.first == Some(position) || self.second == Some(position) {
            return;
        }
        match (self.first, self.second) {
            (None, _) => self.first = Some(position),
            (Some(_), None) => self.second = Some(position),
            (Some(_), Some(_)) => {
                self.first = Some(position);
                self.second = None;
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn selection_range(&self) -> Option<(Position, Position)> {
        let (first, second) = (self.first?, self.second?);
        if first <= second {
            Some((first, second))
        } else {
            Some((second, first))
        }
    }

    pub fn contains(&self, position: Position) -> bool {
        self.selection_range()
            .is_some_and(|(start, end)| position >= start && position < end)
    }

    pub fn marker_at(&self, line: usize) -> Option<char> {
        match (self.first, self.second) {
            (Some(position), _) if position.line == line => Some('-'),
            (_, Some(position)) if position.line == line => Some('+'),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markers_form_an_order_independent_half_open_range() {
        let mut markers = BlockMarkers::default();
        markers.set(Position { line: 2, column: 1 });
        markers.set(Position { line: 0, column: 3 });
        assert_eq!(
            markers.selection_range(),
            Some((
                Position { line: 0, column: 3 },
                Position { line: 2, column: 1 }
            ))
        );
        assert!(markers.contains(Position { line: 1, column: 0 }));
        assert!(!markers.contains(Position { line: 2, column: 1 }));
    }
}
