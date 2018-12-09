#[derive(Debug, PartialEq)]
pub struct Header {
    pub children: u32,
    pub metadata: u32,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub children: Vec<Node>,
    pub metadata: Vec<u32>,
}

pub fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

pub fn get_header(parsed: &[u32]) -> Header {
    let children = parsed.first().unwrap();
    let metadata = parsed.iter().nth(1).unwrap();

    Header { children: *children, metadata: *metadata }
}

pub fn get_length_of_children(node: &Node) -> usize {
    let len = node.children
        .iter()
        .fold(0, |acc, n| {
            let length_of_children = get_length_of_children(&n) + 2;

            acc + length_of_children + n.metadata.len()
        });

    len
}

pub fn get_child(parsed: &[u32]) -> Node {
    let mut node = Node { children: Vec::new(), metadata: Vec::new() };
    let header = get_header(parsed);

    if header.children > 0 {
        for _ in 0..header.children {
            let length_of_children = get_length_of_children(&node) + 2;

            let child = get_child(&parsed[length_of_children..]);
            node.children.push(child);
        }
    }

    let length_of_children = get_length_of_children(&node) + 2;
    let metadata_end = length_of_children + header.metadata as usize;

    for i in &parsed[length_of_children..metadata_end] {
        node.metadata.push(*i);
    }
    node
}

pub fn input_to_nodes(parsed: &[u32]) -> Node {
    let child = get_child(parsed);

    child
}

pub fn count_metadata(node: &Node) -> u32 {
    let children_metadata = node
        .children
        .iter()
        .fold(0, |total, child| {
            let result = count_metadata(child);
            total + result
        });

    let metadata: u32 = node.metadata.iter().sum();

    children_metadata + metadata
}

pub fn count_metadata_with_references(node: &Node) -> u32 {
    let meta: u32 = node.metadata
        .iter()
        .map(|m| {
            if node.children.len() > 0 {
                match node.children.get((*m - 1) as usize) {
                    Some(child) => {
                        count_metadata_with_references(&child)
                    }
                    None => 0
                }
            } else {
                *m
            }
        })
        .sum();

    meta
}

#[cfg(test)]
mod tests {
    use crate::get_header;
    use crate::parse;
    use crate::input_to_nodes;
    use crate::Node;
    use crate::Header;
    use crate::count_metadata;
    use crate::count_metadata_with_references;

    #[test]
    fn it_should_get_header() {
        // Arrange
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        // Act
        let result = get_header(&parse(&input));

        // Assert
        assert_eq!(result, Header { children: 2, metadata: 3 });
    }

    #[test]
    fn it_should_parse_nodes() {
        // Arrange
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let expected_result = Node {
            children: vec![
                Node { children: vec![], metadata: vec![10, 11, 12] },
                Node {
                    children: vec![
                        Node { children: vec![], metadata: vec![99] }
                    ],
                    metadata: vec![2],
                }],
            metadata: vec![1, 1, 2],
        };

        // Act
        let result = input_to_nodes(&parse(&input));

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_should_count_metadata() {
        // Arrange
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let expected_result = 138;

        // Act
        let result = count_metadata(&input_to_nodes(&parse(&input)));

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn it_should_count_metadata_with_references() {
        // Arrange
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let expected_result = 66;

        // Act
        let result = count_metadata_with_references(&input_to_nodes(&parse(&input)));

        // Assert
        assert_eq!(result, expected_result);
    }
}
