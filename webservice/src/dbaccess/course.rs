use crate::errors::MyError;
use crate::models::course::*;
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;

pub async fn get_courses_for_teacher_db(
    pool: &MySqlPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM courses WHERE teacher_id = ?",
        teacher_id
    )
    .fetch_all(pool)
    .await;

    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let rows: Course = sqlx::query_as!(
        Course,
        "SELECT * FROM courses WHERE teacher_id = ? AND id = ?",
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = rows {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".to_string()))
    }
}

pub async fn post_new_course_db(pool: &MySqlPool, new_course: Course) -> Result<Course, MyError> {
    let rows: Course = sqlx::query_as!(
        Course,
        "INSERT INTO courses (teacher_id, name, time) VALUES (?, ?, ?)",
        new_course.teacher_id,
        new_course.name,
        new_course.time
    )
    .fetch_one(pool)
    .await?;

    Ok(rows)
}

pub async fn delete_course_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
) -> Result<Course, MyError> {
    let course_row = sqlx::query!(
        "DELETE FROM courses WHERE teacher_id = ? and id = ?",
        teacher_id,
        id
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record ", course_row))
}

pub async fn update_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
    course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        "UPDATE courses SET name = ?, description = ?, format = ? WHERE teacher_id = ? AND id = ?",
        course.name,
        course.description,
        course.format,
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| MyError::NotFound("Course not found".to_string()));

    let name:String = if let Some(name) = course.name {
        name
    } else {
        current_course_row.name
    };

    let description:String = if let Some(description) = course.description {
        description
    } else {
        current_course_row.description
    };
    
    let format:String = if let Some(format) = course.format {
        format
    } else {
        current_course_row.format
    };

    let course_row = sqlx::query_as!(
        Course,
        "UPDATE courses SET name = ?, description = ?, format = ? WHERE teacher_id = ? AND id = ?",
        name,
        description,
        format,
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await?;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".to_string()))
    }
}
