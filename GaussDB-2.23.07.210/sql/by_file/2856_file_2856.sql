-- 来源: 2856_file_2856.txt
-- SQL 数量: 2

SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ));

SELECT ts_headline ( 'english' , 'The most common type of search is to find all documents containing given query terms and return them in order of their similarity to the query.' , to_tsquery ( 'english' , 'query & similarity' ), 'StartSel = <, StopSel = >' );

