# Mimi IME

Mimi (Meme hay MeoMeo hay Mimi tuỳ cách bạn gọi) là bộ gõ tiếng Việt được viết bằng Rust cho
Linux và môi trường Wayland!!! >;3

**⚠️ Lưu ý:** Đây là dự án nghiên cứu hơn là có thể sử dụng hằng ngày nên có thể bị thiếu nhiều tính năng
và có thể phát sinh nhiều lỗi. Hãy cân nhắc trước khi sử dụng.

## Tính năng

Những tính năng quan trọng sẽ hỗ trợ trong tương lai

- [x] Systray icon
- [x] Hỗ trợ Wayland (chỉ hỗ trợ các compositor hỗ trợ zwp_input_method_v2)
- [ ] Tài liệu giải thích chi tiết của dự án
- [ ] Cải thiện xử lý các sự kiện của bàn phím và phím tắt
- [ ] Cho phép english mode và VNI mode
- [ ] Có GUI cài đặt và thiết lập cấu hình
- [ ] Có icon và logo

## Câu hỏi thường gặp

### Các dự án bộ gõ khác thì sao?

Dự án khác mình không thấy có dự án bộ gõ nào khác có tiềm năng nên rất tiếc mình không thể đề xuất
vì fcitx5 hay ibus đều là bọc từ 1 phần mềm lên tự tạo cho bản thân 1 input method nhưng nó rất khó
hiểu và trải nghiệm phát triển cũng như tài liệu rất tệ nên đây là lý do dự án này được sinh ra để có
thể dễ dàng kiểm soát dự án, có toolchain vui vẻ để phát triển và dễ theo chuẩn Wayland protocols.

### Sẽ có GNOME không?

Có hoặc không tùy thuộc vào tiến độ dự án và cảm hứng của mình. Đây là dự án tự mình làm cho mình nên
sẽ không có nhu cầu sử dụng GNOME trong tương lai gần. Nói về vấn đề input method wayland thì nó rất tệ vì mỗi bên tự tạo
ra tiêu chuẩn nên dự án này sẽ chỉ hỗ trợ được phần nào đó thôi.

### Sẽ có hỗ trợ Xorg chứ?

Không. Ít nhất ở thời điểm hiện tại không có lý do nào để chuyển sang Xorg cả nên dự án ưu tiên Wayland. Nhưng tuỳ vào Xorg
có còn tốt hơn Wayland không hay là 1 codebase không thể duy trì đầy lỗi bảo mật. Xlibre? không một ai biết thế giới sẽ chao đảo
thế nào nữa.

### Được sử dụng AI để code chứ?

Được và dự án này mình cũng xài AI để code. Thứ duy nhất mình quan tâm đó là nó phải hợp lý đúng logic code ít nhất cũng phải dễ hiểu
không được tào lao nếu nó không biên dịch được. Nếu PR bạn quá rác thì ít nhất hãy làm sao cho nó hoạt động. Còn nếu bạn chỉ trích AI slop
xin lỗi bạn có quyền tạo dự án mới và đọc các tiêu chuẩn để dành nhiều thời gian hơn cho dự án của bạn. Mình không quan tâm miễn sao mình
có thể gõ tiếng Việt.
