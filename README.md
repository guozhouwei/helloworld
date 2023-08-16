# 第三课作业
## 请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作

代码位置：https://github.com/guozhouwei/helloworld/blob/master/src/main.rs

## 执行效果：（采用与命令行交互的方式）
### 如图
![avatar](https://github.com/guozhouwei/near_crowdfunding/blob/master/images/%E5%86%B3%E7%AD%96%E6%B5%81%E7%A8%8B%E5%9B%BE.png)

### 命令行日志：
```shell
►►►► 第一步 创建课程 ◄◄◄◄
请输入课程名:
高等数学
是否是必修课（yes/no）:
yes
请输入授课教师:
王老师
是否继续创建课程（yes/no）:
yes
继续创建课程。。。
请输入课程名:
计算机组成原理
是否是必修课（yes/no）:
yes
请输入授课教师:
谭浩强
是否继续创建课程（yes/no）:
yes
继续创建课程。。。
请输入课程名:
体育
是否是必修课（yes/no）:
no
请输入授课教师:
周老师
是否继续创建课程（yes/no）:
no
课程创建完毕！
►►►► 课程列表如下: ◄◄◄◄
编号1, 课程信息：Course { name: "高等数学", compulsory: true, teacher_name: "王老师" }
编号2, 课程信息：Course { name: "计算机组成原理", compulsory: true, teacher_name: "谭浩强" }
编号3, 课程信息：Course { name: "体育", compulsory: false, teacher_name: "周老师" }
►►►► 第二步 创建社团: ◄◄◄◄
请输入社团名称:
舞蹈社团
请输入社团职能:
舞动青春
是否继续创建社团（yes/no）:
yes
继续创建社团。。。
请输入社团名称:
篮球社团
请输入社团职能:
爱运动，做兄弟
是否继续创建社团（yes/no）:
no
社团创建完毕！
►►►► 社团列表如下: ◄◄◄◄
编号1, 社团信息：Club { name: "舞蹈社团", content: "舞动青春" }
编号2, 社团信息：Club { name: "篮球社团", content: "爱运动，做兄弟" }
►►►► 第三步 创建班级: ◄◄◄◄
请输入第几届:
2020
请输入院系:
信息学院
请输入专业:
计算机科学
是否继续创建班级（yes/no）:
no
班级创建完毕！
►►►► 班级列表如下: ◄◄◄◄
编号1, 班级信息：GradeClass { name: "2020届计算机科学班", grade: "2020", faculty: "信息学院", speciality: "计算机科学" }
►►►► 第四步 添加学生（以1个学生为例）: ◄◄◄◄
请输入学生名:
州州
请输入性别(man or female):
man
请输入年龄:
20
请选择下列班级（请输入编号）:
    ->>>> 编号1, 班级名称：GradeClass { name: "2020届计算机科学班", grade: "2020", faculty: "信息学院", speciality: "计算机科学" }
请输入班级编号:
1
请选择下列社团:
    ->>>> 编号1, 社团名称：Club { name: "舞蹈社团", content: "舞动青春" }
    ->>>> 编号2, 社团名称：Club { name: "篮球社团", content: "爱运动，做兄弟" }
请输入社团编号（如若参与多个社团，请逗号分隔）:
1,2
请选择下列课程:
    ->>>> 编号1, 课程名称：Course { name: "高等数学", compulsory: true, teacher_name: "王老师" }
    ->>>> 编号2, 课程名称：Course { name: "计算机组成原理", compulsory: true, teacher_name: "谭浩强" }
    ->>>> 编号3, 课程名称：Course { name: "体育", compulsory: false, teacher_name: "周老师" }
请输入课程编号（如若参与多个课程，请逗号分隔）:
1,2,3
►►►► 学生信息如下: ◄◄◄◄
Student { name: "州州", age: 20, gender: Male, gradeclass: GradeClass { name: "2020届计算机科学班", grade: "2020", faculty: "信息学院", speciality: "计算机科学" }, clubs: [Club { name: "舞蹈社团", content: "舞动青春" }, Club { name: "篮球社团", content: "爱运动，做兄弟" }], courses: [Course { name: "高等数学", compulsory: true, teacher_name: "王老师" }, Course { name: "计算机组成原理", compulsory: true, teacher_name: "谭浩强" }, Course { name: "体育", compulsory: false, teacher_name: "周老师" }] }
►►►► 第五步 修改学生信息 ◄◄◄◄
►►►► 修改学生名称->韩梅梅 ◄◄◄◄
►►►► 修改学生性别->女 ◄◄◄◄
►►►► 删除课程1◄◄◄◄
►►►► 修改后的学生信息如下: ◄◄◄◄
Student { name: "韩梅梅", age: 20, gender: Female, gradeclass: GradeClass { name: "2020届计算机科学班", grade: "2020", faculty: "信息学院", speciality: "计算机科学" }, clubs: [Club { name: "舞蹈社团", content: "舞动青春" }], courses: [Course { name: "高等数学", compulsory: true, teacher_name: "王老师" }, Course { name: "计算机组成原理", compulsory: true, teacher_name: "谭浩强" }, Course { name: "体育", compulsory: false, teacher_name: "周老师" }] }
```