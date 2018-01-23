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
  ((cond (test => result))
   (let ((temp test))
     (if temp (result temp))))
  ((cond (test => result) clause1 clause2 ...)
   (let ((temp test))
     (if temp
       (result temp)
       (cond clause1 clause2 ...))))
  ((cond (test)) test)
  ((cond (test) clause1 clause2 ...)
   (let ((temp test))
     (if temp
       temp
       (cond clause1 clause2 ...))))
  ((cond (test result1 result2 ...))
   (if test (do result1 result2 ...)))
  ((cond (test result1 result2 ...)
         clause1 clause2 ...)
   (if test
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

(defsyntax defn () (
  ((defn name args body ...)
   (def name (fn args body ...)))))

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
