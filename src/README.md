# 第三课作业：请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作

代码位置：https://github.com/guozhouwei/helloworld/blob/master/src/main.rs

执行效果：
```shell
►►►► 第一步 创建课程 ◄◄◄◄
请输入课程名:
数学
是否是必修课（yes/no）:
yes
请输入授课教师:
李丹
是否继续创建课程（yes/no）:
yes
继续创建课程。。。
请输入课程名:
化学
是否是必修课（yes/no）:
no
请输入授课教师:
 王明
是否继续创建课程（yes/no）:
yes
继续创建课程。。。
请输入课程名:
历史
是否是必修课（yes/no）:
no
请输入授课教师:
周正
是否继续创建课程（yes/no）:
no
课程创建完毕！
►►►► 课程列表如下: ◄◄◄◄
编号1, 课程信息：Course { name: "数学", compulsory: true, teacher_name: "李丹" }
编号2, 课程信息：Course { name: "化学", compulsory: false, teacher_name: "王明" }
编号3, 课程信息：Course { name: "历史", compulsory: false, teacher_name: "周正" }
►►►► 第二步 创建社团: ◄◄◄◄
请输入社团名称:
篮球社团
请输入社团职能:
打篮球，爱运动
是否继续创建社团（yes/no）:
yes
继续创建社团。。。
请输入社团名称:
舞蹈社团
请输入社团职能:
舞出青春
是否继续创建社团（yes/no）:
no
社团创建完毕！
►►►► 社团列表如下: ◄◄◄◄
编号1, 社团信息：Club { name: "篮球社团", content: "打篮球，爱运动" }
编号2, 社团信息：Club { name: "舞蹈社团", content: "舞出青春" }
►►►► 第三步 创建班级: ◄◄◄◄
请输入第几届:
2023
请输入院系:
信息学院
请输入专业:
计算机
是否继续创建班级（yes/no）:
yes
继续创建班级。。。
请输入第几届:
2020
请输入院系:
历史学院
请输入专业:
外国   
是否继续创建班级（yes/no）:
yes
继续创建班级。。。
请输入第几届:
2018
请输入院系:
外语学院
请输入专业:
俄语
是否继续创建班级（yes/no）:
no
班级创建完毕！
►►►► 班级列表如下: ◄◄◄◄
编号1, 班级信息：GradeClass { name: "2023届计算机班", grade: "2023", faculty: "信息学院", speciality: "计算机" }
编号2, 班级信息：GradeClass { name: "2020届班", grade: "2020", faculty: "历史学院", speciality: "" }
编号3, 班级信息：GradeClass { name: "2018届俄语班", grade: "2018", faculty: "外语学院", speciality: "俄语" }
►►►► 第四步 添加学生（以1个学生为例）: ◄◄◄◄
请输入学生名:
州州
请输入性别(man or female):
man
请输入年龄:
18
请选择下列班级（请输入编号）:
    ->>>> 编号1, 班级名称：GradeClass { name: "2023届计算机班", grade: "2023", faculty: "信息学院", speciality: "计算机" }
    ->>>> 编号2, 班级名称：GradeClass { name: "2020届班", grade: "2020", faculty: "历史学院", speciality: "" }
    ->>>> 编号3, 班级名称：GradeClass { name: "2018届俄语班", grade: "2018", faculty: "外语学院", speciality: "俄语" }
请输入班级编号:
1
请选择下列社团:
    ->>>> 编号1, 社团名称：Club { name: "篮球社团", content: "打篮球，爱运动" }
    ->>>> 编号2, 社团名称：Club { name: "舞蹈社团", content: "舞出青春" }
请输入社团编号（如若参与多个社团，请逗号分隔）:
2
请选择下列课程:
    ->>>> 编号1, 课程名称：Course { name: "数学", compulsory: true, teacher_name: "李丹" }
    ->>>> 编号2, 课程名称：Course { name: "化学", compulsory: false, teacher_name: "王明" }
    ->>>> 编号3, 课程名称：Course { name: "历史", compulsory: false, teacher_name: "周正" }
请输入课程编号（如若参与多个课程，请逗号分隔）:
1,2,3

```