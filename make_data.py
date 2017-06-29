from sqlalchemy import create_engine, MetaData, Table
DATABASE_URL = "mysql+pymysql://root:mysqlstorm@localhost:3306/pocket"

engine = create_engine(DATABASE_URL)
metadata = MetaData(bind=engine)

# use reflection to construct tables
# will be difficult to do database schema modification
Post = Table('post', metadata, autoload=True)


def create_post(user_id, url, title, content):
    from time import time
    now = time()
    sql = Post.insert().values({
        'user_id': user_id,
        'url': url,
        'title': title,
        'content': content,
        'created_at': now,
        'updated_at': now
    })
    res = engine.execute(sql)
    post_id = res.inserted_primary_key[0]
    return post_id

def get_posts():
    sql = Post.select().where(Post.c.id > 0)
    return engine.execute(sql).fetchall()
