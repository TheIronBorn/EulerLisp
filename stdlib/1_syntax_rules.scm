; (defsyntax and () (
;   ((and) #t)
;   ((and test) test)
;   ((and test1 test2 ...)
;    (if test1 (and test2 ...) #f))))
(defsyntax and () (
  ((and) #t)
  ((and test1) (if test1 #t #f))
  ((and test1 test2 ...)
   (if test1 (and test2 ...) #f))))

(defsyntax defn () (
  ((defn name args body ...)
   (def name (fn args body ...)))))

; TODO: The additional `let` makes this much slower
; (defsyntax or () (
;   ((or) #f)
;   ((or test) test)
;   ((or test1 test2 ...)
;    (let (x test1)
;    (if x x (or test2 ...))))))
(defsyntax or () (
  ((or) #f)
  ((or test1) (if test1 #t #f))
  ((or test1 test2 ...)
   (if test1 #t (or test2 ...)))))

(defsyntax cond (else =>) (
  ((cond (else result1 result2 ...))
   (do result1 result2 ...))
  ((cond (test1 => result))
   (let ((temp test1))
     (if temp (result temp))))
  ((cond (test2 => result) clause1 clause2 ...)
   (let ((temp test2))
     (if temp
       (result temp)
       (cond clause1 clause2 ...))))
  ((cond (test3)) test3)
  ((cond (test4) clause1 clause2 ...)
   (let ((temp test4))
     (if temp
       temp
       (cond clause1 clause2 ...))))
  ((cond (test5 result1 result2 ...))
   (if test5 (do result1 result2 ...)))
  ((cond (test6 result1 result2 ...)
         clause1 clause2 ...)
   (if test6
     (do result1 result2 ...)
     (cond clause1 clause2 ...)))))

; TODO: R5RS has a version that is more complex,
; does it have any features that would be nice to replicate?
(defsyntax case () (
  ((case (key ...) clauses ...)
   (let ((atom-key (key ...)))
     (case atom-key clauses ...)))
  ((case key (else consq)) consq)
  ((case key (atom consq)) (if (= key atom) consq))
  ((case key (atom consq) rest ...)
   (if (= key atom)
       consq
       (case key rest ...)))))

(defsyntax let () (
  ((let () body ...) (do body ...))
  ((let ((name val) ...) body1 body2 ...)
   ((fn (name ...) body1 body2 ...) val ...))))

(defsyntax let* () (
  ((let* ()
     body1 body2 ...)
   (let ()
       body1 body2 ...))
  ((let* ((name1 val1) rest ...)
     body1 body2 ...)
   (let ((name1 val1))
     (let* (rest ...)
       body1 body2 ...)))))

(defsyntax when () (
  ((when test body ...)
   (if test (do body ...)))))

(defsyntax unless () (
  ((unless test conseq)
   (if test '() conseq))
  ((unless test conseq alt)
   (if test alt conseq))))

(defsyntax ~> () (
  ((~> first) first)
  ((~> first (second args ...))
   (second args ... first))
  ((~> first second)
   (second first))
  ((~> first (second args ...) rest ...)
   (~> (second args ... first) rest ...))
  ((~> first second rest ...)
   (~> (second first) rest ...))))

; (defsyntax letrec_generate_temp_names () (
;   ((letrec_generate_temp_names
;      ()
;      (temp1 ...)
;      ((var1 init1) ...)
;      body ...)
;    (let ((var1 <undefined>) ...)
;      (let ((temp1 init1) ...)
;        (set! var1 temp1)
;        ...
;        body ...)))
;   ((letrec_generate_temp_names
;      (x y ...)
;      (temp ...)
;      ((var1 init1) ...)
;      body ...)
;    (letrec_generate_temp_names
;      (y ...)
;      (newtemp temp ...)
;      ((var1 init1) ...)
;      body ...))))

; (defsyntax letrec () (
;   ((letrec ((var1 init1) ...) body ...)
;    (letrec_generate_temp_names
;      (var1 ...)
;      ()
;      ((var1 init1) ...)
;      body ...))))

; (defsyntax for_step () (
;   ((for_step x) x)
;   ((for_step x y) y)))

; (defsyntax for () (
;   ((do ((var init step ...) ...)
;        (test expr ...)
;        command ...)
;    (letrec
;      ((loop
;         (lambda (var ...)
;                 (if test
;                   (do
;                     (if #f #f)
;                     expr ...)
;                   (do
;                     command ...
;                     (loop (for_step var step ...) ...))))))
;      (loop init ...)))))
