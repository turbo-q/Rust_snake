rust 版本贪吃蛇小游戏



todo list
- [ ] STEP 只能为1，这个不是大问题
- [ ] ✌️判定
- [ ] 增加开机动画
- [ ] 发射子弹功能，发射子弹后击中可以吞食物
- [ ] 如果发射子弹后没有击中则子弹就会无限回弹，蛇头需要躲避子弹
- [x] 反向移动应该panic




## 迭代P
修改为基于事件驱动的，根据不同事件做不同的事

# README
在窗口绘画本身就是相对于窗口左上角的位置，不需要计算偏移量
方向优先解决移动和方向改变的丝滑性：方向改变后立即渲染一次，但是放弃原本的移动渲染