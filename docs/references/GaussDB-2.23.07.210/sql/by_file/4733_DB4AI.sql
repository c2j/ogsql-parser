-- 来源: 4733_DB4AI.txt
-- SQL 数量: 16

CREATE MODEL iris_classification_model USING xgboost_regression_logistic FEATURES sepal_length, sepal_width,petal_length,petal_width TARGET target_type < 2 FROM tb_iris_1 WITH nthread=4, max_depth=8;

select gs_explain_model('iris_classification_model');

SELECT id , PREDICT BY iris_classification (FEATURES sepal_length,sepal_width,petal_length,petal_width ) as " PREDICT" FROM tb_iris limit 3;

Explain CREATE MODEL patient_logisitic_regression USING logistic_regression FEATURES second_attack, treatment TARGET trait_anxiety > 50 FROM patients WITH batch_size=10, learning_rate = 0.05;

CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET trait_anxiety FROM patients WITH optimizer='aa';

CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET trait_anxiety FROM patients;

CREATE MODEL patient_linear_regression USING linear_regression FEATURES * TARGET trait_anxiety FROM patients;

-----------------------------------------------------------------------------------------------------------------------
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET * FROM patients;

CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment FROM patients;

CREATE MODEL ecoli_svmc USING multiclass FEATURES f1, f2, f3, f4, f5, f6, f7 TARGET cat FROM (SELECT * FROM db4ai_ecoli WHERE cat='cp');

create model iris_classification_model using xgboost_regression_logistic features message_regular target error_level from error_code;

CREATE MODEL ecoli_svmc USING multiclass FEATURES f1, f2, f3, f4, f5, f6, f7, cat TARGET cat FROM db4ai_ecoli ;

select gs_explain_model("ecoli_svmc");

select id, PREDICT BY patient_logistic_regression (FEATURES second_attack,treatment) FROM patients;

select id, PREDICT BY patient_linear_regression (FEATURES second_attack) FROM patients;

-------------------------------------------------------------------------------------------------------------------------------------
select id, PREDICT BY patient_linear_regression (FEATURES 1,second_attack,treatment) FROM patients;

