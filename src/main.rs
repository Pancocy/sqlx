use::std::io;
use::std::env;
use dotenv::dotenv;
use::chrono::NaiveDateTime;
use::sqlx::postgres::PgPoolOptions;


#[derive(Debug)]
pub struct Course{
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>
}

#[actix_rt::main]
async  fn main() -> io::Result<()>{
    //读取存储与.env中的数据库连接信息
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("'DATABASE_URL'未在.env文件中正确设置");

    //创建数据库连接
    let data_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    //查询语句与结果
    let course_rowdata = sqlx::query!(
        r#"select id,teacher_id,name,time from course where id = $1"#,
        2
    )
    .fetch_all(&data_pool)
    .await
    .unwrap();

    //存储并输出数据
    let mut course_list = vec![];
    for row in course_rowdata {
        course_list.push(Course{
            id:row.id,
            teacher_id:row.teacher_id,
            name:row.name,
            time:Some(NaiveDateTime::from(row.time.unwrap())),
        })
    }
    println!("counses = {:?}",course_list);
    Ok(())
}

