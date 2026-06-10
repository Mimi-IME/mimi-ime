![MimiIME Logo](images/mimi-ime-logo1.svg)

Mimi IME (Meme hay MeoMeo hay Mimi tuỳ cách bạn gọi) là bộ gõ tiếng Việt được viết bằng Rust 🦀 cho
Linux và môi trường Wayland!!! >;3

Mimi IME sẽ tuân theo chuẩn của Wayland nên sẽ chỉ có duy nhất 1 chế độ pre-edit commit để tránh
race condition, làm hỏng tiếng trình gõ text của bạn và sẽ cố gắng nhẹ nhất có thể.

**Lưu ý:** Hiện Mimi IME chỉ hỗ trợ build package từ nix flake cài đặt thành home-manager hay NixOS
service. Nếu bạn muốn hỗ trợ trên distro của bạn, mình sẽ không chịu trách nhiệm cho việc đó. Giải
pháp còn lại bạn cài đặt từ mã nguồn hoặc tự bạn đóng gói nó cho distro của bạn lại. Vì sao lại thế
hãy xem thêm ở [FAQ](./docs/faq.md).

## Hỗ trợ

Nếu bạn cần trợ giúp có thể tạo báo cáo lỗi hoặc vào phòng chat trực tuyến Matrix ở đây.

[Tham gia phòng Matrix](https://matrix.to/#/#mimi-ime:matrix.org)

**Lưu ý:** Nếu có thể hãy đọc [FAQ](./docs/faq.md), [Why Mimi IME?](./docs/why-mimi-ime.md) và
[Project Overview](./docs/project-overview.md) trước khi gửi báo cáo lỗi

## Đóng góp

Nếu bạn muốn đóng góp đừng ngần ngại tạo báo cáo lỗi hay gửi code trên PR cho mình.

Đọc tài liệu tổng quan dự án ở [đây](./docs/project-overview.md).

## Tính năng

Các tính năng cơ bản đã được hoàn thành.

- [x] Systray icon
- [x] Hỗ trợ Wayland (chỉ hỗ trợ các compositor hỗ trợ zwp_input_method_v2)
- [x] Cho phép english mode và VNI mode
- [x] Có icon và logo
- [x] Cải thiện xử lý các sự kiện của bàn phím và phím tắt
- [x] Tài liệu giải thích chi tiết của dự án
- [x] Có GUI cài đặt và thiết lập cấu hình

Nếu bạn cảm thấy chưa đủ có thể đề xuất tính năng mới thông qua báo cáo lỗi. Tuy nhiên, dự án
cũng sẽ có những định kiến riêng hy vọng bạn thông cảm nếu tính năng bạn đề xuất không phù hợp
với dự án. Hãy đọc thêm ở mục [FAQ](./docs/faq.md).
