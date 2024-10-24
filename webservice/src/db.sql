
DATABASE_URL=mariadb://root:123456@127.0.0.1:3306/my-rust_web


-- drop table if exists courses;
create table courses (
    id serial primary key,
    teacher_id int not null,
    name VARCHAR(140) not null,
    time timestamp default now()
);

insert into courses (id, teacher_id, name,time) values (1, 1, 'Advanced Programming in Rust', '2024-01-01 11:00:00');
insert into courses (id, teacher_id, name,time) values (2, 1, 'Advanced Programming in Rust', '2024-01-01 12:00:00');

127.0.0.1:3306

