/*!
* Grammar (will keep changing):
*
*
*   command := WORD+ redirect*
*
*   redirect := stdout_to
*           |   stdout_app
*           |   stdout_from
*           |   stderr_to
*           |   stderr_app
*
*       stdout_to := '1'? '>' WORD
*       stdout_app := '1'? '>>' WORD
*       stdout_from := <' WORD
*       stderr_to := '2' '>' WORD
*       stderr_app := '2' '>>' WORD
*
*   WORD := TOKEN::Word(String)
*
*
*/
