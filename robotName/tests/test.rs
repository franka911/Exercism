#![allow(warnings)]
extern crate robotName as robot;

#[cfg(test)]
mod tests{

    #[test]
    fn testNewNamesSameRobot (){
        let mut robot1 = robot::Robot::new();
        let name1 = robot1.getName().to_string();
        robot1.reset();
        let name2 = robot1.getName().to_string();
        assert_ne!(name1, name2);
    }

    #[test]
    fn testNewNamesDifferentRobots (){
        let robot1 = robot::Robot::new();
        let name1 = robot1.getName();
        let robot2 = robot::Robot::new();
        let name2 = robot2.getName();
        assert_ne!(name1, name2);
    }

}