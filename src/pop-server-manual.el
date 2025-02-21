;;; pop-server

;; Copyright (C) 2025 - Josep Bigorra, jjba23 <jjbigorra@gmail.com>

;; pop-server is free software: you can redistribute it and/or modify
;; it under the terms of the Lesser GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; pop-server is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the Lesser GNU General Public License
;; along with pop-server.  If not, see <https://www.gnu.org/licenses/>.

;;; Code:

(defun pop-server/publish-readme ()
  "Publish the POP-SERVER readme from Org to several formats.
This assumes that the current directory is the parent folder"
  (find-file "README.org")
  (org-md-export-to-markdown)
  (message "finished publishing pop-server readme"))
