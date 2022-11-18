#!/usr/bin/rs

(load "tool")
(load "opt")

; https://leetcode.com/problems/longest-substring-without-repeating-characters/

(defun lengthOfLongestSubstring ((?s string))

    (defvar ?len (length-of ?s))
    (defvar ?b (make-array 256))
    
    ; init all array to [0,0,...0]
    (loop for ?i from 0 to 255 do 
        (seta ?b ?i 0)
    )

    (defvar ?last 0)
    (defvar ?maxLen 0)
    
    (loop for ?i from 0 to (- ?len 1) do 
        (defvar ?c (str-char-at ?s ?i))
        (defvar ?idx (get ?b ?c))
        (if (> ?idx ?last) do
            (setq ?maxLen (max (- ?i ?last) ?maxLen))
            (setq ?last ?idx)
        )
        (seta ?b ?c (+ ?i 1))
    )

    (return (max (- ?len ?last) ?maxLen))

)[opt-full]

(defun test ((?s string))
    (defvar ?r  (lengthOfLongestSubstring ?s))    
    (println "input:=" ?s ", output=" ?r)
)

(test "abcabcbb")
