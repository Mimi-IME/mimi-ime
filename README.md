# Mimi IME

Đây là dự án nghiên cứu hơn là có thể sử dụng hằng ngày nên có thể bị thiếu nhiều tính năng.
Hãy cân nhắc trước khi sử dụng.

## Tính năng

- [x] Systray icon
- [x] Hỗ trợ Wayland (chỉ hỗ trợ các compositor hỗ trợ zwp_input_method_v2)
- [ ] Tài liệu giải thích chi tiết của dự án
- [ ] Cải thiện xử các sự kiện của phím tắt
- [ ] Cho phép english mode và VNI mode
- [ ] Có settings GUI
- [ ] Có icon và logo

## Câu hỏi thường gặp

Q: Sẽ có GNOME không?
A: Có hoặc không tùy thuộc vào tiến độ dự án và cảm hứng của mình. Đây là dự án tự mình làm cho mình nên
sẽ không có nhu cầu sử dụng GNOME trong tương lai gần. Nói về vấn đề input method wayland thì nó rất tệ vì mỗi bên tự tạo
ra tiêu chuẩn nên dự án này sẽ chỉ hỗ trợ được phần nào đó thôi.

Q: Sẽ có hỗ trợ Xorg chứ?
A: Không. Ít nhất ở thời điểm hiện tại không có lý do nào để chuyển sang Xorg cả nên dự án ưu tiên Wayland. Nhưng tuỳ vào Xorg
có còn tốt hơn Wayland không hay là 1 codebase không thể duy trì đầy lỗi bảo mật. Xlibre? không một ai biết thế giới sẽ chao đảo
thế nào nữa.

Q: Được sử dụng AI để code chứ?
A: Được và dự án này mình cũng xài AI để code. Thứ duy nhất mình quan tâm đó là nó phải hợp lý đúng logic. Nếu PR bạn quá rác
thì ít nhất hãy làm sao cho nó hoạt động. Còn nếu bạn chỉ trích AI slop xin lỗi bạn có quyền tạo dự án mới và đọc các tiêu chuẩn
để dành nhiều thời gian hơn cho dự án của bạn. Mình không quan tâm miễn sao mình có thể gõ tiếng Việt.
