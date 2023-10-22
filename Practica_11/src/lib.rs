// Definimos una estructura para el nodo de la lista enlazada
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Definimos una estructura para la lista enlazada
#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    // Constructor de la lista vacía
    pub fn new() -> Self {
        List { head: None }
    }

    // Agregar un elemento al final de la lista
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });
        let mut current = &mut self.head;
        while let Some(ref mut node) = *current {
            current = &mut node.next;
        }
        *current = Some(new_node);
    }

    // Eliminar todos los elementos de la lista
    pub fn clear(&mut self) {
        self.head = None;
    }

    // Obtener la longitud de la lista
    pub fn len(&self) -> usize {
        let mut current = &self.head;
        let mut count = 0;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    // Obtener el valor en la posición index
    pub fn index(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        let mut current_index = 0;
        while let Some(node) = current {
            if current_index == index {
                return Some(&node.value);
            }
            current_index += 1;
            current = &node.next;
        }
        None
    }

    // Insertar un valor en la posición index
    pub fn insert(&mut self, index: usize, value: T) {
        if index == 0 {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            let mut current_index = 0;
            while let Some(ref mut node) = *current {
                if current_index + 1 == index {
                    let new_node = Box::new(Node {
                        value,
                        next: node.next.take(),
                    });
                    node.next = Some(new_node);
                    return;
                }
                current_index += 1;
                current = &mut node.next;
            }
            self.push(value); // Si el índice es mayor que la longitud, simplemente lo agregamos al final
        }
    }

    // Eliminar el valor en la posición index
    pub fn remove(&mut self, index: usize) {
        if index == 0 {
            self.head = self.head.take().and_then(|node| node.next);
        } else {
            let mut current = &mut self.head;
            let mut current_index = 0;
            while let Some(ref mut node) = *current {
                if current_index + 1 == index {
                    node.next = node.next.take().and_then(|node| node.next);
                    return;
                }
                current_index += 1;
                current = &mut node.next;
            }
        }
    }

    // Revertir la lista
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }

    // Obtener el valor en la posición index
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        let mut current_index = 0;
        while let Some(node) = current {
            if current_index == index {
                return Some(&node.value);
            }
            current_index += 1;
            current = &node.next;
        }
        None
    }
}