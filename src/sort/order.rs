
/**
Represents an order or sorting.
*/
pub enum SortingOrder {
    /// Ascending order, for example ```[1,2,2,3,4,5]```.
    Ascending,
    /// Descending order, for example ```[5,4,3,2,2,1]```.
    Descending
}

/**
Represents ordering of the given sorted collection.
*/
pub enum Order {
    ///Increasing order, for example ```[1,2,3,4,5]```.
    Increasing,
    ///Decreasing order, for example ```[5,4,3,2,1]```.
    Decreasing,
    ///Not increasing order, for example ```[5,5,4,4,1]```.
    NotIncreasing,
    ///Not decreasing order, for example ```[1,1,2,2,3]```.
    NotDecreasing
}